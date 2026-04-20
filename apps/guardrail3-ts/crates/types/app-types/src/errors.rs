use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error returned when crawling the target workspace fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceCrawlError {
    /// Human-readable failure message for CLI and renderer output.
    pub message: String,
}

impl Display for WorkspaceCrawlError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for WorkspaceCrawlError {}

/// Error returned when a family runner cannot produce findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyRunError {
    /// Human-readable failure message for CLI and renderer output.
    pub message: String,
}

impl Display for FamilyRunError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for FamilyRunError {}
