use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use futures::stream::{self, Stream};
use reqwest::Client;
use thiserror::Error;
use tokio::{io::AsyncWriteExt, sync::RwLock};
use tracing::instrument;

use crate::endpoint::{AuthPolicy, Endpoint, EndpointDeserializeError, EndpointPaginated};
use crate::endpoint_group::EndpointGroup;

#[derive(Debug, Error)]
pub enum EndpointGroupClientError<E: std::error::Error + Send + Sync + 'static> {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] EndpointDeserializeError),
    #[error("Authentication error: {0}")]
    Auth(Box<dyn std::error::Error + Send + Sync>),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("API error: {0}")]
    Api(E),
}
type EGCResult<R, E> = Result<R, EndpointGroupClientError<E>>;

#[derive(Debug, Clone)]
pub struct EndpointGroupClientConfig {
    pub timeout_sec: u64,
}

impl EndpointGroupClientConfig {
    pub fn build_client(&self) -> Result<Client, reqwest::Error> {
        Client::builder()
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .zstd(true)
            .timeout(Duration::from_secs(self.timeout_sec))
            .build()
    }
}

impl Default for EndpointGroupClientConfig {
    fn default() -> Self {
        Self { timeout_sec: 30 }
    }
}

pub struct EndpointGroupClient<G: EndpointGroup> {
    http: Client,
    group: G,
    auth_info_cached: Arc<RwLock<Option<G::AuthInfo>>>,
}

type BoxedEGCResultStream<'a, R, E> = Box<dyn Stream<Item = EGCResult<R, E>> + Send + 'a>;

impl<G: EndpointGroup> EndpointGroupClient<G> {
    pub fn new(http: Client, group: G) -> Self {
        Self {
            http,
            group,
            auth_info_cached: Arc::new(RwLock::new(None)),
        }
    }

    async fn fetch_auth_info<E>(&self) -> EGCResult<G::AuthInfo, E>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let auth_info = self.auth_info_cached.read().await.clone();
        if let Some(auth_info) = auth_info
            && self.group.is_valid_auth(&auth_info)
        {
            return Ok(auth_info);
        }
        let auth_info = self
            .group
            .fetch_auth_info(self)
            .await
            .map_err(EndpointGroupClientError::Auth)?;
        *self.auth_info_cached.write().await = Some(auth_info.clone());
        Ok(auth_info)
    }

    async fn send<E, A>(
        &self,
        endpoint: &E,
        req: &E::Request,
        auth_info: Option<&G::AuthInfo>,
    ) -> EGCResult<reqwest::Response, E::Error>
    where
        E: Endpoint<A>,
        A: AuthPolicy,
    {
        let url = format!("{}{}", self.group.base_url(), endpoint.path(req));
        let mut builder = self.http.request(endpoint.method(), &url);
        let query_params = endpoint.query(req);
        if !query_params.is_empty() {
            builder = builder.query(&query_params);
        }
        if A::REQUIRES_AUTH
            && let Some(auth_info) = auth_info
        {
            builder = self.group.apply_auth(builder, auth_info);
        }
        Ok(endpoint.body(builder, req).send().await?)
    }

    async fn send_with_retry<E, A>(
        &self,
        endpoint: &E,
        req: &E::Request,
        auth_info: Option<&G::AuthInfo>,
    ) -> EGCResult<reqwest::Response, E::Error>
    where
        E: Endpoint<A>,
        A: AuthPolicy,
    {
        let mut attempt = 0_u32;
        loop {
            let res = self.send::<E, A>(endpoint, req, auth_info).await?;
            if self.group.should_retry(res.status()) && attempt < self.group.max_retry() {
                let wait = self.group.retry_backoff_base_ms() * (1 << attempt);
                tokio::time::sleep(Duration::from_millis(wait)).await;
                attempt += 1;
                continue;
            }
            return Ok(res);
        }
    }

    async fn fetch<E, A>(
        &self,
        endpoint: &E,
        req: &E::Request,
    ) -> EGCResult<E::OkResponse, E::Error>
    where
        E: Endpoint<A>,
        A: AuthPolicy,
    {
        let auth_info = if A::REQUIRES_AUTH {
            Some(self.fetch_auth_info().await?)
        } else {
            None
        };
        let res = self
            .send_with_retry::<E, A>(endpoint, req, auth_info.as_ref())
            .await?;

        let res = if A::REQUIRES_AUTH
            && let Some(auth_info) = auth_info
            && self.group.should_refresh_auth(&auth_info, res.status())
        {
            *self.auth_info_cached.write().await = None;
            let auth_info = self.fetch_auth_info().await?;
            self.send_with_retry::<E, A>(endpoint, req, Some(&auth_info))
                .await?
        } else {
            res
        };

        if res.status().is_success() {
            endpoint
                .parse_response::<E::OkResponse>(res)
                .await
                .map_err(EndpointGroupClientError::Deserialize)
        } else {
            let status = res.status();
            let err_res = endpoint
                .parse_response::<E::ErrResponse>(res)
                .await
                .map_err(EndpointGroupClientError::Deserialize)?;
            Err(EndpointGroupClientError::Api(
                endpoint.on_err(status, err_res),
            ))
        }
    }

    #[instrument(skip(self))]
    pub async fn execute<E, A>(
        &self,
        endpoint: &E,
        req: &E::Request,
    ) -> EGCResult<E::Response, E::Error>
    where
        E: Endpoint<A>,
        A: AuthPolicy,
    {
        let ok_res = self.fetch::<E, A>(endpoint, req).await?;
        endpoint
            .on_ok(ok_res)
            .map_err(EndpointGroupClientError::Api)
    }

    #[instrument(skip(self))]
    pub fn execute_paginated<'a, E, A>(
        &'a self,
        endpoint: &'a E,
        initial_req: &E::Request,
    ) -> Pin<BoxedEGCResultStream<'a, E::Response, E::Error>>
    where
        E: EndpointPaginated<A> + 'a,
        A: AuthPolicy,
        E::Request: Clone + 'a,
    {
        Box::pin(stream::unfold(
            Some(initial_req.clone()),
            move |state| async {
                let req = state?;
                match self.fetch::<E, A>(endpoint, &req).await {
                    Ok(ok_res) => {
                        let next_req = endpoint.next_request(&req, &ok_res);
                        let res = endpoint
                            .on_ok(ok_res)
                            .map_err(EndpointGroupClientError::Api);
                        Some((res, next_req))
                    }
                    Err(e) => Some((Err(e), None)),
                }
            },
        ))
    }

    #[instrument(skip(self))]
    pub async fn download_file<E>(&self, url: &str, path: &Path) -> EGCResult<(), E>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut res = self.http.get(url).send().await?.error_for_status()?;
        let mut file = tokio::fs::File::create(path).await?;
        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await?;
        }
        file.flush().await?;
        Ok(())
    }
}
