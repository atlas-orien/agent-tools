use axum::{Router, routing::post};

use crate::{handlers::tools::web_search::web_search, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/web-search", post(web_search))
}
