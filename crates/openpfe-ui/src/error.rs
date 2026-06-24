use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use openpfe_llm::LlmError;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: ErrorEnvelope,
}

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    code: &'static str,
    message: String,
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "invalid_request",
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "not_found",
            message: message.into(),
        }
    }

    pub fn model_not_loaded(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "model_not_loaded",
            message: message.into(),
        }
    }

    pub fn inference_busy() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "inference_busy",
            message: "inference busy".into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal_error",
            message: message.into(),
        }
    }
}

impl From<LlmError> for ApiError {
    fn from(err: LlmError) -> Self {
        match err {
            LlmError::Busy => Self::inference_busy(),
            LlmError::NotFound(msg) => Self::not_found(msg),
            LlmError::InvalidRequest(msg) | LlmError::Config(msg) => Self::invalid_request(msg),
            LlmError::Engine(msg) if msg == "no model loaded" => {
                Self::model_not_loaded(msg)
            }
            LlmError::Engine(msg) => Self::model_not_loaded(msg),
            LlmError::Download(msg) | LlmError::Verify(msg) => Self::invalid_request(msg),
            LlmError::Io(e) => Self::internal(e.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            error: ErrorEnvelope {
                code: self.code,
                message: self.message,
            },
        };
        (self.status, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
