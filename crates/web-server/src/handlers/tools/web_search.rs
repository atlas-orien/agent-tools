use axum::{Json, extract::State};
use toolcraft_axum_kit::{IntoCommonResponse, ResponseResult};

use crate::{
    dto::{WebSearchRequest, WebSearchResponse},
    error::{bad_request, internal},
    state::AppState,
};

pub async fn web_search(
    State(_state): State<AppState>,
    Json(req): Json<WebSearchRequest>,
) -> ResponseResult<WebSearchResponse> {
    let query = req.query.trim().to_string();
    if query.is_empty() {
        return Err(bad_request("query cannot be empty"));
    }

    let answer = tokio::task::spawn_blocking(move || agent_tools::web_search(query))
        .await
        .map_err(|err| internal(err.to_string()))?
        .map_err(|err| internal(err.to_string()))?;

    Ok(WebSearchResponse { answer }
        .into_common_response()
        .to_json())
}
