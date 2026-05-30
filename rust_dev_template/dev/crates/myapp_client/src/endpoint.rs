pub mod code_enum;
pub mod example;
pub mod never_endpoint;

use std::fmt::Debug;
use std::future::Future;

use reqwest::{RequestBuilder, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

pub trait AuthPolicy: Send + Sync {
    const REQUIRES_AUTH: bool;
}

pub struct WithAuth;
impl AuthPolicy for WithAuth {
    const REQUIRES_AUTH: bool = true;
}

pub struct NoAuth;
impl AuthPolicy for NoAuth {
    const REQUIRES_AUTH: bool = false;
}

#[derive(Debug, Error)]
pub enum EndpointDeserializeError {
    #[error("Retrieve response body error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Json parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub trait Endpoint<A: AuthPolicy>: Send + Sync + Debug {
    type Request: Serialize + Send + Sync + Debug;
    type OkResponse: DeserializeOwned + Send + Debug;
    type ErrResponse: DeserializeOwned + Send + Debug;
    type Response: Send + Debug;
    type Error: std::error::Error + Send + Sync + 'static;

    fn method(&self) -> reqwest::Method;
    fn path(&self, req: &Self::Request) -> String;
    fn query(&self, _req: &Self::Request) -> Vec<(String, String)> {
        vec![]
    }
    fn body(&self, builder: RequestBuilder, req: &Self::Request) -> RequestBuilder {
        builder.json(req)
    }

    fn parse_response<R>(
        &self,
        res: reqwest::Response,
    ) -> impl Future<Output = Result<R, EndpointDeserializeError>> + Send
    where
        R: DeserializeOwned + Send,
    {
        async move {
            let bytes = res.bytes().await?;
            tracing::debug!(
                "Response[..1024]: {}",
                String::from_utf8_lossy(&bytes)
                    .chars()
                    .take(1024)
                    .collect::<String>()
            );
            Ok(serde_json::from_slice::<R>(&bytes)?)
        }
    }
    fn on_ok(&self, ok: Self::OkResponse) -> Result<Self::Response, Self::Error>;
    fn on_err(&self, status: StatusCode, err: Self::ErrResponse) -> Self::Error;
}

pub trait EndpointPaginated<A: AuthPolicy>: Endpoint<A> {
    fn next_request(
        &self,
        prev_req: &Self::Request,
        prev_ok: &Self::OkResponse,
    ) -> Option<Self::Request>;
}
