use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct WebSearchRequest {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct WebSearchResponse {
    pub answer: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalSearchQuery {
    pub q: String,
    pub topic: Option<String>,
    pub days: Option<u64>,
    pub max_results: Option<u64>,
    pub bypass_cache: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExternalSearchResponse {
    pub status: String,
    pub key: String,
    pub engine: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub data: ExternalSearchData,
}

#[derive(Debug, Serialize)]
pub struct ExternalSearchData {
    pub items: Vec<Value>,
}
