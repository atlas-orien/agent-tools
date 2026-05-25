# agent-tools

Small Rust helpers for running local commands and Codex-powered tools.

## Features

- Run commands with args, cwd, env, stdin, timeout, and output capture.
- Run shell commands when shell syntax is needed.
- Call the local `codex` CLI from Rust.
- Use Codex web search with a simple `web_search(query)` function.

## Install

```toml
[dependencies]
agent-tools = "0.1"
```

## Run a Command

```rust
use agent_tools::{CmdRequest, CmdTool};

let output = CmdTool::run(CmdRequest {
    program: "echo".to_string(),
    args: vec!["hello".to_string()],
    cwd: None,
    env: None,
    timeout_ms: Some(1_000),
    fail_on_non_zero: true,
    stdin: None,
    background: false,
})?;

assert_eq!(output.stdout.trim(), "hello");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Web Search

```rust
use agent_tools::web_search;

let answer = web_search("What is the latest stable Rust version?")?;
println!("{answer}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

Web search uses the local `codex` CLI with `--search`. It does not call a separate search API directly. Make sure `codex` is installed and logged in before using it.

For advanced configuration, use `WebSearchRequest` and `WebSearchTool`.

## Notes

- `run_shell` passes the command string to the system shell. Do not use it with untrusted input.
- Command output is collected into memory.
- Timeout kills the direct child process only; shell child processes may need stronger process-group handling in production.
