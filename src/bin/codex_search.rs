use agent_tools::web_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", web_search(build_query())?);
    Ok(())
}

fn build_query() -> String {
    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if query.trim().is_empty() {
        return "What is the latest stable Rust version today?".to_string();
    }

    query
}
