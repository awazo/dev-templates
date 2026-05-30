use std::future::Future;

use reqwest::{RequestBuilder, StatusCode};

use crate::endpoint::{Endpoint, NoAuth};
use crate::endpoint_group_client::EndpointGroupClient;

pub trait EndpointGroup: Send + Sync + Sized {
    type AuthEndpoint: Endpoint<NoAuth>;
    type AuthInfo: Send + Sync + Clone;

    fn base_url(&self) -> &str;
    fn apply_auth(&self, builder: RequestBuilder, auth: &Self::AuthInfo) -> RequestBuilder;

    fn auth_endpoint(&self) -> Self::AuthEndpoint;
    fn auth_request(&self) -> <Self::AuthEndpoint as Endpoint<NoAuth>>::Request;
    fn extract_auth_info(
        &self,
        res: <Self::AuthEndpoint as Endpoint<NoAuth>>::Response,
    ) -> Self::AuthInfo;
    fn fetch_auth_info(
        &self,
        client: &EndpointGroupClient<Self>,
    ) -> impl Future<Output = Result<Self::AuthInfo, Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        async move {
            let endpoint = self.auth_endpoint();
            let req = self.auth_request();
            let res = client
                .execute(&endpoint, &req)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            Ok(self.extract_auth_info(res))
        }
    }

    fn is_valid_auth(&self, _auth: &Self::AuthInfo) -> bool {
        true
    }
    fn should_refresh_auth(&self, _auth: &Self::AuthInfo, status: StatusCode) -> bool {
        status == StatusCode::UNAUTHORIZED
    }

    fn should_retry(&self, status: StatusCode) -> bool {
        status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
    }
    fn max_retry(&self) -> u32 {
        5
    }
    fn retry_backoff_base_ms(&self) -> u64 {
        500
    }
}
