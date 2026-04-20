/// Final process output for one CLI command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionOutcome {
    /// Text written to stdout.
    stdout: String,
    /// Text written to stderr.
    stderr: String,
    /// Final process exit code.
    exit_code: i32,
}

impl ExecutionOutcome {
    /// Stores the final stdout, stderr, and exit code for the CLI.
    #[must_use]
    pub const fn new(stdout: String, stderr: String, exit_code: i32) -> Self {
        Self {
            stdout,
            stderr,
            exit_code,
        }
    }

    /// Returns stdout as rendered by the report layer.
    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns stderr built from crawl and runner failures.
    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Returns the CLI process exit code.
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        self.exit_code
    }
}
