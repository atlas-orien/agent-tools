use axum::http::StatusCode;
use toolcraft_axum_kit::{ApiError, CommonError};

pub const BAD_REQUEST: i16 = 400;
pub const INTERNAL_ERROR: i16 = 500;

pub fn bad_request(message: impl Into<String>) -> ApiError {
    common_error(StatusCode::BAD_REQUEST, BAD_REQUEST, message)
}

pub fn internal(message: impl Into<String>) -> ApiError {
    common_error(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_ERROR, message)
}

fn common_error(status: StatusCode, code: i16, message: impl Into<String>) -> ApiError {
    (
        status,
        CommonError {
            code,
            message: message.into(),
        }
        .to_json(),
    )
}
