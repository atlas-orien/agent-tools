use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::tools::web_search::{external_search, web_search},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/web-search", post(web_search))
}

pub fn external_search_routes() -> Router<AppState> {
    Router::new().route("/search", get(external_search))
}
