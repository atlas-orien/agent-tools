use toolcraft_axum_kit::{IntoCommonResponse, ResponseResult};

use crate::dto::HealthResponse;

pub async fn health() -> ResponseResult<HealthResponse> {
    Ok(HealthResponse { status: "ok" }
        .into_common_response()
        .to_json())
}
