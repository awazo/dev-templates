use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub body: Option<T>,
}

pub type ApiResponseWithoutBody = ApiResponse<()>;

impl<T> ApiResponse<T> {
    pub fn ok_only() -> Self {
        ApiResponse::ok_only_msg("")
    }

    pub fn ok_only_msg(message: impl Into<String>) -> Self {
        ApiResponse::status_only_msg("OK", message)
    }

    pub fn ok(body: T) -> Self {
        ApiResponse::ok_msg("", body)
    }

    pub fn ok_msg(message: impl Into<String>, body: T) -> Self {
        ApiResponse {
            status: "OK".to_string(),
            message: message.into(),
            body: Some(body),
        }
    }

    pub fn error_only() -> Self {
        ApiResponse::error_only_msg("")
    }

    pub fn error_only_msg(message: impl Into<String>) -> Self {
        ApiResponse::status_only_msg("ERROR", message)
    }

    pub fn status_only(status: impl Into<String>) -> Self {
        ApiResponse::status_only_msg(status, "")
    }

    pub fn status_only_msg(status: impl Into<String>, message: impl Into<String>) -> Self {
        ApiResponse {
            status: status.into(),
            message: message.into(),
            body: None,
        }
    }
}
