use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    CmdOutput, CmdStdin, CodexOptions, CodexRequest, CodexTool, ColorMode, SandboxMode,
    error::{Error, Result},
};

pub struct WebSearchTool;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSearchRequest {
    pub query: String,
    pub timeout_ms: Option<u64>,
    pub codex_options: CodexOptions,
}

impl WebSearchRequest {
    pub fn new(query: impl Into<String>) -> Self {
        let codex_options = CodexOptions {
            search: true,
            ephemeral: true,
            color: Some(ColorMode::Never),
            sandbox: Some(SandboxMode::ReadOnly),
            skip_git_repo_check: true,
            ..CodexOptions::default()
        };

        Self {
            query: query.into(),
            timeout_ms: Some(120_000),
            codex_options,
        }
    }
}

pub fn web_search(query: impl Into<String>) -> Result<String> {
    let output = web_search_output(query)?;
    Ok(output.stdout.trim().to_string())
}

pub fn web_search_output(query: impl Into<String>) -> Result<CmdOutput> {
    WebSearchTool::search(WebSearchRequest::new(query))
}

impl WebSearchTool {
    pub fn search(req: WebSearchRequest) -> Result<CmdOutput> {
        let workdir = create_isolated_workdir()?;
        let mut codex_req = CodexRequest::new(build_web_search_prompt(&req.query));
        codex_req.stdin = Some(CmdStdin::Null);
        codex_req.timeout_ms = req.timeout_ms;
        codex_req.fail_on_non_zero = true;
        codex_req.options = req.codex_options;
        codex_req.options.cd = Some(workdir.display().to_string());

        let result = CodexTool::exec(codex_req);
        let _ = fs::remove_dir_all(workdir);
        result
    }
}

fn build_web_search_prompt(query: &str) -> String {
    format!(
        "Use live web search to answer the user's query.\n\nRequirements:\n- Search the web when \
         needed; prefer primary, official, and recent sources.\n- If the answer depends on \
         current information, state the date you checked.\n- Cite the main web sources used, \
         especially for current, specific, or disputed information.\n- If sources disagree, \
         mention the disagreement briefly.\n- Answer in the same language as the user's query \
         unless the query asks otherwise.\n- Be concise, but include enough detail to make the \
         answer useful.\n- Do not read, inspect, create, edit, or delete local files.\n- Do not \
         run shell commands or use local tools unless they are strictly required for web \
         search.\n\nUser query:\n{query}"
    )
}

fn create_isolated_workdir() -> Result<PathBuf> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let dir = std::env::temp_dir().join(format!(
        "agent-tools-web-search-{}-{unique}",
        std::process::id()
    ));

    fs::create_dir(&dir).map_err(Error::tool_io)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_request_defaults() {
        let req = WebSearchRequest::new("latest rust release");

        assert_eq!(req.query, "latest rust release");
        assert_eq!(req.timeout_ms, Some(120_000));
        assert!(req.codex_options.search);
        assert!(req.codex_options.ephemeral);
        assert!(req.codex_options.skip_git_repo_check);
        assert!(req.codex_options.model.is_none());
        assert_eq!(req.codex_options.sandbox, Some(SandboxMode::ReadOnly));
        assert!(req.codex_options.cd.is_none());
    }

    #[test]
    fn test_web_search_output_uses_simple_query_defaults() {
        let req = WebSearchRequest::new("macbook pro price");

        assert_eq!(req.query, "macbook pro price");
        assert!(req.codex_options.search);
        assert!(req.codex_options.model.is_none());
    }

    #[test]
    fn test_web_search_prompt_is_general() {
        let prompt = build_web_search_prompt("最新 Rust 版本是什么？");

        assert!(prompt.contains("Use live web search"));
        assert!(prompt.contains("same language as the user's query"));
        assert!(prompt.contains("Do not read, inspect, create, edit, or delete local files"));
        assert!(prompt.contains("Do not run shell commands"));
        assert!(prompt.contains("最新 Rust 版本是什么？"));
    }

    #[test]
    fn test_isolated_workdir_is_created_outside_current_project() {
        let dir = create_isolated_workdir().unwrap();

        assert!(dir.exists());
        assert!(dir.starts_with(std::env::temp_dir()));

        fs::remove_dir_all(dir).unwrap();
    }
}
