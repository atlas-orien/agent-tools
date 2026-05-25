mod tools;

use std::{fmt, io};

/// Core error struct for the workspace.
/// This hides the internal details of specific errors (like ToolError).
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorKind>,
}

/// Internal enum representing all possible error variants.
/// This is not exposed publicly, preventing users from matching on internal details.
#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error(transparent)]
    Tools(#[from] tools::ToolError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl Error {
    /// Creates an IO error associated with tools execution.
    pub fn tool_io(err: io::Error) -> Self {
        Error {
            inner: Box::new(ErrorKind::Tools(tools::ToolError::Io(err))),
        }
    }

    /// Creates a command failure error associated with tools execution.
    pub fn tool_cmd_failed(code: i32) -> Self {
        Error {
            inner: Box::new(ErrorKind::Tools(tools::ToolError::CommandFailed(code))),
        }
    }

    /// Creates a timeout error associated with tools execution.
    pub fn tool_timeout() -> Self {
        Error {
            inner: Box::new(ErrorKind::Tools(tools::ToolError::Timeout)),
        }
    }
}

/// Result type alias using the opaque Error.
pub type Result<T> = std::result::Result<T, Error>;
