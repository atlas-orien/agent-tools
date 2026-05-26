mod dto;
mod error;
mod handlers;
mod routes;
mod settings;
mod state;

use crate::{routes::create_routes, settings::Settings, state::AppState};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("server failed: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load("config/services.toml")?;
    let bind = format!("0.0.0.0:{}", settings.http.port);

    let state = AppState::new(&settings);
    let app = create_routes(state);

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    println!("agent-tools-server listening on http://{}", bind);

    axum::serve(listener, app).await.map_err(Into::into)
}
