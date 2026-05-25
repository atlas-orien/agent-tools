use serde::{Deserialize, Serialize};

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
