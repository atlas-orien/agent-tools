use axum::{
    Json,
    extract::{Query, State},
};
use serde_json::Value;
use toolcraft_axum_kit::{ApiError, IntoCommonResponse, ResponseResult};

use crate::{
    dto::{
        ExternalSearchData, ExternalSearchQuery, ExternalSearchResponse, WebSearchRequest,
        WebSearchResponse,
    },
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

pub async fn external_search(
    State(_state): State<AppState>,
    Query(req): Query<ExternalSearchQuery>,
) -> ResponseResult<ExternalSearchResponse> {
    let query = req.q.trim().to_string();
    if query.is_empty() {
        return Err(bad_request("q cannot be empty"));
    }

    let topic = req.topic.unwrap_or_else(|| "general".to_string());
    let days = req.days.unwrap_or(30).clamp(1, 365);
    let max_results = req.max_results.unwrap_or(6).clamp(1, 12);
    let bypass_cache = req.bypass_cache.unwrap_or(false);

    let output = tokio::task::spawn_blocking(move || {
        agent_tools::external_search_provider_output(query, topic, days, max_results, bypass_cache)
    })
    .await
    .map_err(|err| internal(err.to_string()))?
    .map_err(|err| internal(err.to_string()))?;

    let response = parse_external_search_response(&output.stdout)?;
    Ok(response.into_common_response().to_json())
}

fn parse_external_search_response(stdout: &str) -> Result<ExternalSearchResponse, ApiError> {
    let value: Value = serde_json::from_str(stdout.trim())
        .map_err(|err| internal(format!("codex search output is not valid JSON: {err}")))?;
    let object = value
        .as_object()
        .ok_or_else(|| internal("codex search output must be a JSON object"))?;

    let items = value
        .pointer("/data/items")
        .and_then(Value::as_array)
        .ok_or_else(|| internal("codex search output must contain data.items array"))?
        .clone();

    Ok(ExternalSearchResponse {
        status: object
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("live")
            .to_string(),
        key: object
            .get("key")
            .and_then(Value::as_str)
            .unwrap_or("search")
            .to_string(),
        engine: object
            .get("engine")
            .and_then(Value::as_str)
            .unwrap_or("codex-web-search")
            .to_string(),
        requested_at: object
            .get("requested_at")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        confidence: object.get("confidence").and_then(Value::as_f64),
        data: ExternalSearchData {
            items: items.into_iter().collect(),
        },
    })
}
