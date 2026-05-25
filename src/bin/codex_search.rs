use agent_tools::{WebSearchRequest, WebSearchTool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut req = WebSearchRequest::new(build_query());

    req.timeout_ms = Some(read_timeout_ms());

    if let Ok(model) = std::env::var("CODEX_MODEL") {
        if !model.trim().is_empty() {
            req.codex_options.model = Some(model);
        }
    }

    let output = WebSearchTool::search(req)?;

    if !output.stdout.trim().is_empty() {
        println!("{}", output.stdout.trim());
    }
    if !output.stderr.trim().is_empty() {
        eprintln!("{}", output.stderr.trim());
    }

    Ok(())
}

fn build_query() -> String {
    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if query.trim().is_empty() {
        return "What is the latest stable Rust version today?".to_string();
    }

    query
}

fn read_timeout_ms() -> u64 {
    std::env::var("CODEX_SEARCH_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(120_000)
}
