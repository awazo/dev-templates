pub mod code;
pub mod example_endpoint_directory;

use std::future::Future;

use reqwest::{RequestBuilder, StatusCode};
use serde::Deserialize;
use thiserror::Error;

use crate::endpoint::never_endpoint::{NeverEndpoint, NeverRequest};
use crate::endpoint::{Endpoint, NoAuth};
use crate::endpoint_group::EndpointGroup;
use crate::endpoint_group_client::EndpointGroupClient;

pub struct ExampleEndpointGroup {
    pub base_url: String,
    pub api_key: String,
    pub max_retry: u32,
    pub retry_backoff_base_ms: u64,
}

impl EndpointGroup for ExampleEndpointGroup {
    type AuthEndpoint = NeverEndpoint;
    type AuthInfo = String;

    fn base_url(&self) -> &str {
        &self.base_url
    }
    fn apply_auth(&self, builder: RequestBuilder, auth: &Self::AuthInfo) -> RequestBuilder {
        builder.header("x-api-key", auth)
    }

    fn auth_endpoint(&self) -> Self::AuthEndpoint {
        Self::AuthEndpoint {}
    }
    fn auth_request(&self) -> <Self::AuthEndpoint as Endpoint<NoAuth>>::Request {
        NeverRequest
    }
    fn extract_auth_info(
        &self,
        _res: <Self::AuthEndpoint as Endpoint<NoAuth>>::Response,
    ) -> Self::AuthInfo {
        unreachable!()
    }
    fn fetch_auth_info(
        &self,
        _client: &EndpointGroupClient<Self>,
    ) -> impl Future<Output = Result<Self::AuthInfo, Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        let api_key = self.api_key.clone();
        async move { Ok(api_key) }
    }

    fn should_refresh_auth(&self, _auth: &Self::AuthInfo, status: StatusCode) -> bool {
        status == StatusCode::BAD_REQUEST
            || status == StatusCode::UNAUTHORIZED
            || status == StatusCode::FORBIDDEN
    }

    fn max_retry(&self) -> u32 {
        self.max_retry
    }
    fn retry_backoff_base_ms(&self) -> u64 {
        self.retry_backoff_base_ms
    }
}

#[derive(Debug, Deserialize)]
pub struct ExampleErrResponse {
    pub message: String,
}

#[derive(Debug, Error)]
pub enum ExampleError {
    #[error("[{status}] {message}")]
    Api { status: StatusCode, message: String },
}

impl ExampleError {
    pub fn from_response(status: StatusCode, res: ExampleErrResponse) -> Self {
        Self::Api {
            status,
            message: res.message,
        }
    }
}
