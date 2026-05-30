use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{Endpoint, NoAuth};

#[derive(Debug)]
pub struct NeverEndpoint;
#[derive(Serialize, Default, Debug)]
pub struct NeverRequest;
#[derive(Deserialize, Debug)]
pub struct NeverResponse;
#[derive(Debug, Error)]
pub struct NeverError;
impl std::fmt::Display for NeverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NeverEndpoint should never be called")
    }
}

impl Endpoint<NoAuth> for NeverEndpoint {
    type Request = NeverRequest;
    type OkResponse = NeverResponse;
    type ErrResponse = NeverResponse;
    type Response = NeverResponse;
    type Error = NeverError;

    fn method(&self) -> reqwest::Method {
        unreachable!()
    }
    fn path(&self, _req: &NeverRequest) -> String {
        unreachable!()
    }

    fn on_ok(&self, _: Self::OkResponse) -> Result<Self::Response, Self::Error> {
        unreachable!()
    }
    fn on_err(&self, _: reqwest::StatusCode, _: Self::ErrResponse) -> Self::Error {
        unreachable!()
    }
}
