use std::path::PathBuf;

use crate::{CmdOutput, CmdRequest, CmdStdin, CmdTool, error::Result};

pub struct CodexTool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxMode {
    ReadOnly,
    WorkspaceWrite,
    DangerFullAccess,
}

impl SandboxMode {
    fn as_str(self) -> &'static str {
        match self {
            SandboxMode::ReadOnly => "read-only",
            SandboxMode::WorkspaceWrite => "workspace-write",
            SandboxMode::DangerFullAccess => "danger-full-access",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OssProvider {
    LmStudio,
    Ollama,
}

impl OssProvider {
    fn as_str(self) -> &'static str {
        match self {
            OssProvider::LmStudio => "lmstudio",
            OssProvider::Ollama => "ollama",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Always,
    Never,
    Auto,
}

impl ColorMode {
    fn as_str(self) -> &'static str {
        match self {
            ColorMode::Always => "always",
            ColorMode::Never => "never",
            ColorMode::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CodexRequest {
    pub task: Option<String>,
    pub stdin: Option<CmdStdin>,
    pub timeout_ms: Option<u64>,
    pub fail_on_non_zero: bool,
    pub background: bool,
    pub dangerously_bypass_approvals_and_sandbox: bool,
    pub options: CodexOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CodexOptions {
    pub search: bool,
    pub config: Vec<String>,
    pub enable: Vec<String>,
    pub disable: Vec<String>,
    pub images: Vec<PathBuf>,
    pub model: Option<String>,
    pub oss: bool,
    pub local_provider: Option<OssProvider>,
    pub sandbox: Option<SandboxMode>,
    pub profile: Option<String>,
    pub full_auto: bool,
    pub dangerously_bypass_approvals_and_sandbox: bool,
    pub cd: Option<String>,
    pub skip_git_repo_check: bool,
    pub add_dirs: Vec<String>,
    pub ephemeral: bool,
    pub output_schema: Option<String>,
    pub color: Option<ColorMode>,
    pub progress_cursor: bool,
    pub json: bool,
    pub output_last_message: Option<String>,
}

impl CodexRequest {
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            task: Some(task.into()),
            stdin: None,
            timeout_ms: None,
            fail_on_non_zero: true,
            background: false,
            dangerously_bypass_approvals_and_sandbox: false,
            options: CodexOptions::default(),
        }
    }

    pub fn from_stdin(stdin: CmdStdin) -> Self {
        Self {
            task: None,
            stdin: Some(stdin),
            timeout_ms: None,
            fail_on_non_zero: true,
            background: false,
            dangerously_bypass_approvals_and_sandbox: false,
            options: CodexOptions::default(),
        }
    }
}

impl CodexTool {
    pub fn exec(req: CodexRequest) -> Result<CmdOutput> {
        Self::run_args(
            build_exec_args(&req),
            req.stdin.clone(),
            req.timeout_ms,
            req.fail_on_non_zero,
            req.background,
        )
    }

    fn run_args(
        args: Vec<String>,
        stdin: Option<CmdStdin>,
        timeout_ms: Option<u64>,
        fail_on_non_zero: bool,
        background: bool,
    ) -> Result<CmdOutput> {
        CmdTool::run(CmdRequest {
            program: "codex".to_string(),
            args,
            cwd: None,
            env: None,
            timeout_ms,
            fail_on_non_zero,
            stdin,
            background,
        })
    }
}

fn build_exec_args(req: &CodexRequest) -> Vec<String> {
    let mut args = Vec::new();
    push_common_args(
        &mut args,
        req.options.search,
        &req.options.config,
        &req.options.enable,
        &req.options.disable,
        &req.options.images,
        req.options.model.as_deref(),
        req.options.oss,
        req.options.local_provider,
        req.options.sandbox,
        req.options.profile.as_deref(),
        req.options.full_auto,
        req.dangerously_bypass_approvals_and_sandbox
            || req.options.dangerously_bypass_approvals_and_sandbox,
        req.options.cd.as_deref(),
        &req.options.add_dirs,
    );
    args.push("exec".to_string());

    if req.options.skip_git_repo_check {
        args.push("--skip-git-repo-check".to_string());
    }
    if req.options.ephemeral {
        args.push("--ephemeral".to_string());
    }
    if let Some(output_schema) = &req.options.output_schema {
        args.push("--output-schema".to_string());
        args.push(output_schema.clone());
    }
    if let Some(color) = req.options.color {
        args.push("--color".to_string());
        args.push(color.as_str().to_string());
    }
    if req.options.progress_cursor {
        args.push("--progress-cursor".to_string());
    }
    if req.options.json {
        args.push("--json".to_string());
    }
    if let Some(output_last_message) = &req.options.output_last_message {
        args.push("--output-last-message".to_string());
        args.push(output_last_message.clone());
    }

    push_prompt_arg(&mut args, req.task.as_deref(), req.stdin.as_ref());
    args
}

#[allow(clippy::too_many_arguments)]
fn push_common_args(
    args: &mut Vec<String>,
    search: bool,
    config: &[String],
    enable: &[String],
    disable: &[String],
    images: &[PathBuf],
    model: Option<&str>,
    oss: bool,
    local_provider: Option<OssProvider>,
    sandbox: Option<SandboxMode>,
    profile: Option<&str>,
    full_auto: bool,
    dangerously_bypass_approvals_and_sandbox: bool,
    cd: Option<&str>,
    add_dirs: &[String],
) {
    if search {
        args.push("--search".to_string());
    }

    for entry in config {
        args.push("-c".to_string());
        args.push(entry.clone());
    }
    for feature in enable {
        args.push("--enable".to_string());
        args.push(feature.clone());
    }
    for feature in disable {
        args.push("--disable".to_string());
        args.push(feature.clone());
    }
    for image in images {
        args.push("--image".to_string());
        args.push(image.display().to_string());
    }
    if let Some(model) = model {
        args.push("--model".to_string());
        args.push(model.to_string());
    }
    if oss {
        args.push("--oss".to_string());
    }
    if let Some(local_provider) = local_provider {
        args.push("--local-provider".to_string());
        args.push(local_provider.as_str().to_string());
    }
    if let Some(sandbox) = sandbox {
        args.push("--sandbox".to_string());
        args.push(sandbox.as_str().to_string());
    }
    if let Some(profile) = profile {
        args.push("--profile".to_string());
        args.push(profile.to_string());
    }
    if full_auto {
        args.push("--full-auto".to_string());
    }
    if dangerously_bypass_approvals_and_sandbox {
        args.push("--dangerously-bypass-approvals-and-sandbox".to_string());
    }
    if let Some(cd) = cd {
        args.push("--cd".to_string());
        args.push(cd.to_string());
    }
    for dir in add_dirs {
        args.push("--add-dir".to_string());
        args.push(dir.clone());
    }
}

fn push_prompt_arg(args: &mut Vec<String>, prompt: Option<&str>, stdin: Option<&CmdStdin>) {
    if let Some(prompt) = prompt {
        args.push(prompt.to_string());
        return;
    }

    if stdin.is_some() {
        args.push("-".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_exec_args_with_structured_options() {
        let req = CodexRequest {
            task: Some("run tests".to_string()),
            stdin: None,
            timeout_ms: Some(10_000),
            fail_on_non_zero: true,
            background: false,
            dangerously_bypass_approvals_and_sandbox: true,
            options: CodexOptions {
                search: true,
                config: vec!["model=\"gpt-5\"".to_string()],
                enable: vec!["fast_mode".to_string()],
                disable: vec!["slow_mode".to_string()],
                images: vec![PathBuf::from("/tmp/a.png")],
                model: Some("gpt-5".to_string()),
                oss: true,
                local_provider: Some(OssProvider::Ollama),
                sandbox: Some(SandboxMode::WorkspaceWrite),
                profile: Some("default".to_string()),
                full_auto: true,
                dangerously_bypass_approvals_and_sandbox: false,
                cd: Some("/tmp/work".to_string()),
                skip_git_repo_check: true,
                add_dirs: vec!["/tmp/extra".to_string()],
                ephemeral: true,
                output_schema: Some("/tmp/schema.json".to_string()),
                color: Some(ColorMode::Never),
                progress_cursor: true,
                json: true,
                output_last_message: Some("/tmp/out.txt".to_string()),
            },
        };

        let args = build_exec_args(&req);
        assert_eq!(args[0], "--search");
        assert!(args.iter().any(|x| x == "exec"));
        assert!(args.contains(&"--skip-git-repo-check".to_string()));
        assert!(args.contains(&"--ephemeral".to_string()));
        assert!(args.contains(&"--json".to_string()));
        assert!(args.contains(&"--dangerously-bypass-approvals-and-sandbox".to_string()));
        assert!(args.contains(&"run tests".to_string()));
    }

    #[test]
    fn test_build_exec_args_uses_stdin_marker_without_prompt() {
        let req = CodexRequest {
            stdin: Some(CmdStdin::Text("hello".to_string())),
            ..CodexRequest::default()
        };

        let args = build_exec_args(&req);
        assert_eq!(args, vec!["exec".to_string(), "-".to_string()]);
    }

    #[test]
    fn test_new_request_defaults_to_simple_task() {
        let req = CodexRequest::new("fix tests");
        let args = build_exec_args(&req);

        assert_eq!(args, vec!["exec".to_string(), "fix tests".to_string()]);
        assert!(req.fail_on_non_zero);
        assert!(!req.background);
        assert!(!req.options.search);
    }

    #[test]
    fn test_top_level_dangerous_flag_is_applied() {
        let mut req = CodexRequest::new("do work");
        req.dangerously_bypass_approvals_and_sandbox = true;

        let args = build_exec_args(&req);
        assert!(args.contains(&"--dangerously-bypass-approvals-and-sandbox".to_string()));
    }
}
