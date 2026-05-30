use reqwest::{Method, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

use super::super::{ExampleErrResponse, ExampleError, code::example_code::ExampleCode};
use crate::endpoint::{
    Endpoint, EndpointPaginated, WithAuth,
    code_enum::{CodeEnum, code_enum_str_opt_serde, code_enum_str_serde},
};

#[derive(Clone, Debug, Serialize)]
pub struct ExampleEndpointRequest {
    #[serde(
        deserialize_with = "code_enum_str_opt_serde::deserialize",
        serialize_with = "code_enum_str_opt_serde::serialize",
        default
    )]
    pub example: Option<ExampleCode>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub pagination_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExampleEndpointResponse {
    pub data: Vec<ExampleData>,
    pub pagination_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExampleData {
    pub date: String,
    #[serde(with = "code_enum_str_serde", default)]
    pub example: ExampleCode,
}

#[derive(Debug)]
pub struct ExampleEndpoint;

impl Endpoint<WithAuth> for ExampleEndpoint {
    type Request = ExampleEndpointRequest;
    type OkResponse = ExampleEndpointResponse;
    type ErrResponse = ExampleErrResponse;
    type Response = ExampleEndpointResponse;
    type Error = ExampleError;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self, _req: &Self::Request) -> String {
        "/example_directory/example".to_string()
    }
    fn query(&self, req: &Self::Request) -> Vec<(String, String)> {
        let mut q: Vec<(String, String)> = vec![];
        if let Some(example) = req.example {
            q.push(("example".into(), example.code().into()));
        }
        if let Some(from) = &req.from {
            q.push(("from".into(), from.clone()));
        }
        if let Some(to) = &req.to {
            q.push(("to".into(), to.clone()));
        }
        if let Some(pagination_key) = &req.pagination_key {
            q.push(("pagination_key".into(), pagination_key.clone()));
        }
        q
    }
    fn body(&self, builder: RequestBuilder, _req: &Self::Request) -> RequestBuilder {
        builder
    }

    fn on_ok(&self, ok: Self::OkResponse) -> Result<Self::Response, Self::Error> {
        Ok(ok)
    }
    fn on_err(&self, status: StatusCode, err: Self::ErrResponse) -> Self::Error {
        Self::Error::from_response(status, err)
    }
}

impl EndpointPaginated<WithAuth> for ExampleEndpoint {
    fn next_request(
        &self,
        prev_req: &Self::Request,
        prev_ok: &Self::OkResponse,
    ) -> Option<Self::Request> {
        prev_ok.pagination_key.as_ref().map(|key| Self::Request {
            pagination_key: Some(key.clone()),
            ..(*prev_req).clone()
        })
    }
}
