# `tools` crate README

## 概述

`tools` crate 当前提供了一个轻量级命令执行模块 `cmd`，用于在 Rust 代码中启动外部进程、传入工作目录和环境变量、写入标准输入、设置超时，以及按前台或后台模式运行命令。

当前公开接口位于：

- `tools::cmd::CmdTool`
- `tools::cmd::CmdRequest`
- `tools::cmd::ShellCmdRequest`
- `tools::cmd::CmdOutput`
- `tools::cmd::CmdStdin`

其中，crate 根还额外 re-export 了：

- `tools::CmdTool`
- `tools::CmdRequest`
- `tools::ShellCmdRequest`
- `tools::CmdOutput`
- `tools::CmdStdin`

## 适用场景

这个模块适合以下场景：

- 在业务代码中统一封装简单的命令调用
- 向命令注入少量文本、字节或文件作为 `stdin`
- 对短时命令设置执行超时
- 启动无需采集输出的后台任务

如果你的需求包含以下能力，当前实现还不够完整，建议扩展后再用于关键链路：

- 实时流式读取 `stdout` / `stderr`
- 可靠执行高输出量命令
- 强约束 shell 安全边界
- 后台任务生命周期管理
- 跨平台一致的进程组终止语义
- 二进制输出的无损采集

## API 说明

### `CmdRequest`

`CmdRequest` 用于执行普通命令，也就是明确区分 `program + args` 的场景：

```rust
pub struct CmdRequest {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub timeout_ms: Option<u64>,
    pub fail_on_non_zero: bool,
    pub stdin: Option<CmdStdin>,
    pub background: bool,
}
```

字段说明：

- `program`：要执行的程序名。
- `args`：参数列表。
- `cwd`：子进程工作目录。
- `env`：额外环境变量。
- `timeout_ms`：超时时间，单位毫秒。
- `fail_on_non_zero`：是否将非零退出码视为错误。
- `stdin`：标准输入来源。
- `background`：是否后台启动。后台模式下当前实现不会采集输出。

### `ShellCmdRequest`

`ShellCmdRequest` 用于执行整段 shell 命令字符串：

```rust
pub struct ShellCmdRequest {
    pub command: String,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub timeout_ms: Option<u64>,
    pub fail_on_non_zero: bool,
    pub stdin: Option<CmdStdin>,
    pub background: bool,
}
```

字段说明：

- `command`：整段 shell 命令。
- 其余字段含义与 `CmdRequest` 一致。

### `CmdStdin`

`stdin` 支持三种来源：

```rust
pub enum CmdStdin {
    Text(String),
    Bytes(Vec<u8>),
    File(PathBuf),
}
```

- `Text`：按 UTF-8 文本写入。
- `Bytes`：按原始字节写入。
- `File`：将文件句柄直接绑定为子进程标准输入。

### `CmdOutput`

```rust
pub struct CmdOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub pid: Option<u32>,
}
```

字段说明：

- `stdout`：前台执行时采集到的标准输出。
- `stderr`：前台执行时采集到的标准错误。
- `exit_code`：进程退出码。若因信号退出，当前实现会返回 `-1`。
- `pid`：仅后台模式返回子进程 PID。

## 使用方式

### 前台执行

```rust
use tools::{CmdRequest, CmdTool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    assert_eq!(output.exit_code, 0);
    assert_eq!(output.stdout.trim(), "hello");
    Ok(())
}
```

### 使用 shell

当命令包含管道、重定向、通配符或 shell 内建语法时，使用 `CmdTool::run_shell`：

```rust
use tools::{CmdTool, ShellCmdRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = CmdTool::run_shell(ShellCmdRequest {
        command: "echo 'hello world' | grep world".to_string(),
        cwd: None,
        env: None,
        timeout_ms: Some(1_000),
        fail_on_non_zero: true,
        stdin: None,
        background: false,
    })?;

    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("world"));
    Ok(())
}
```

注意：

- 非 Windows 平台使用 `sh -c`，Windows 使用 `cmd.exe /c`。
- 推荐默认优先使用 `CmdTool::run`，只有确实需要 shell 语法时才用 `run_shell`。

### 通过 `stdin` 传文本

```rust
use tools::cmd::CmdStdin;
use tools::{CmdRequest, CmdTool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = CmdTool::run(CmdRequest {
        program: "cat".to_string(),
        args: vec![],
        cwd: None,
        env: None,
        timeout_ms: Some(1_000),
        fail_on_non_zero: true,
        stdin: Some(CmdStdin::Text("hello stdin".to_string())),
        background: false,
    })?;

    assert_eq!(output.stdout, "hello stdin");
    Ok(())
}
```

### 后台执行

```rust
use tools::{CmdRequest, CmdTool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = CmdTool::run(CmdRequest {
        program: "sleep".to_string(),
        args: vec!["10".to_string()],
        cwd: None,
        env: None,
        timeout_ms: None,
        fail_on_non_zero: false,
        stdin: None,
        background: true,
    })?;

    assert!(output.pid.is_some());
    Ok(())
}
```

注意：

- 后台模式只表示“成功启动”，不表示任务已执行成功。
- 当前实现不会跟踪后台任务，也不会主动回收其退出状态。

## 错误语义

当前 crate 使用 `error` crate 的统一错误类型，`CmdTool::run` 和 `CmdTool::run_shell` 主要可能返回以下错误：

- IO 错误：命令不存在、文件打不开、进程创建失败、等待失败等
- 超时错误：超过 `timeout_ms` 后返回 `Command timed out`
- 非零退出码错误：当 `fail_on_non_zero = true` 时，返回 `Command failed with exit code N`

退出码语义如下：

- `fail_on_non_zero = false`：即使退出码非 0，也返回 `Ok(CmdOutput)`，由调用方自行检查 `exit_code`
- `fail_on_non_zero = true`：退出码非 0 时返回 `Err`

当前限制：

- 返回 `Err` 时，错误对象里只保留退出码，没有携带 `stdout` / `stderr`
- 如果你的上层需要失败输出，建议后续扩展结构化错误类型

## 生产环境注意事项

### 1. 已修复：大输出命令的阻塞风险

当前实现已经改为并发读取 `stdout` / `stderr`，不再是“先 `wait()` 再读输出”的模型，因此相比之前版本，已经规避了最典型的 pipe buffer 死锁问题。

不过仍有两个边界需要注意：

- 输出会完整读入内存，超大输出仍可能造成内存压力
- 当前没有提供流式消费接口

如果要用于不受控输出的命令，建议再增加输出大小限制或流式回调接口

### 2. 已修复：`stdin` / `stdout` / `stderr` 的 IO 错误静默吞掉

当前实现已经把这些 IO 操作改成显式错误返回，不再静默忽略：

- 写入 `stdin`
- 读取 `stdout`
- 读取 `stderr`

### 3. 部分缓解：非 UTF-8 输出不再直接丢失

当前实现会先按字节读取输出，再使用 `String::from_utf8_lossy` 转成文本。这意味着：

- 非 UTF-8 输出不再因为 `read_to_string` 失败而被直接吞掉
- 非法字节会被替换字符表示，文本查看更稳妥

但它仍然不是“无损二进制输出”方案。如果此模块要支持编译器产物、压缩流、图片或任意二进制命令输出，建议后续补充：

- `stdout_bytes` / `stderr_bytes`
- 或直接把输出主体改为 `Vec<u8>`，文本视图作为辅助层提供

### 4. 超时终止不够彻底

当前超时逻辑只对当前子进程执行 `kill()`。当使用 `run_shell` 时，常见问题是：

- 被 kill 的只是 shell 进程
- shell 拉起的孙子进程可能继续运行

如果需要生产级行为，建议按平台补齐：

- Unix 上使用进程组并 kill 整个进程组
- Windows 上使用 Job Object 或等价机制

### 5. 后台任务缺少生命周期管理

当前后台模式只返回 PID，不提供：

- 查询状态
- 回收退出码
- 终止任务
- 日志路径
- 标准输出持久化

另外，和旧版本相比，当前实现已经修复了一个实际 bug：

- 后台模式下传入 `Text` / `Bytes` 型 `stdin` 时，现在会在返回前先完成写入

如果上层真的要用它跑后台任务，建议增加一个任务句柄层，而不是直接暴露裸 PID。

### 6. shell 入口的安全边界需要明确

当前 `run_shell` 会把完整字符串交给 shell 解释。这意味着：

- 存在命令注入风险

建议在 API 层明确约束：

- 默认禁止不受信任输入进入 `command`
- README 和 doc comment 中明确这是高风险模式

## 建议补齐的能力

如果目标是“可在生产代码中稳定复用”的命令执行模块，建议优先补齐以下能力：

1. 非零退出码策略
2. 失败错误中保留 `stdout` / `stderr`
3. 二进制输出支持
4. 输出大小限制，避免内存被异常放大
5. 进程组级超时终止
6. 后台任务管理接口
7. 更完整的测试覆盖，包括大输出、shell 子进程超时、后台任务回收

## 当前测试覆盖

仓库内现有测试已经覆盖：

- 正常命令执行
- shell 命令执行
- 超时
- 不存在的命令
- `stdin` 文本 / 字节 / 文件
- 后台执行
- shell 管道
- 非零退出码的双模式处理
- 非 UTF-8 输出的 lossy 解码

但仍缺少更关键的生产级测试：

- 大输出下的压力测试
- `stdin` 写入失败时的行为
- shell 派生子进程在超时后的残留情况
- 后台进程退出后的资源回收

## 结论

当前 `cmd` 模块适合做一个轻量、可工作的命令调用封装原型；如果按生产级标准判断，它还没有完全达到“通用、安全、稳定”的命令执行基础设施水平。

这版实现已经把最直接的稳定性问题补上了两项：

- 并发读取输出，避免常见的大输出阻塞
- `stdin/stdout/stderr` 的 IO 错误不再静默吞掉

剩下最需要继续往生产级推进的点是：

- 超时场景下的进程组清理
- 失败错误中保留输出内容
- 后台任务的生命周期管理
