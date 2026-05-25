pub mod tools;

use axum::{Router, routing::get};
use toolcraft_axum_kit::middleware::cors::create_cors;

use crate::{handlers::health::health, state::AppState};

pub fn create_routes(state: AppState) -> Router {
    let cors = create_cors();

    Router::new()
        .route("/health", get(health))
        .nest("/tools", tools::routes())
        .layer(cors)
        .with_state(state)
}
