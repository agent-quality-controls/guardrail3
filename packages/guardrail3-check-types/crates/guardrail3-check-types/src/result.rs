use crate::G3Severity;

/// A single finding from a guardrail3 check.
///
/// Every check package returns `Vec<G3CheckResult>`. The orchestrator
/// collects results from all packages and renders the report.
#[derive(Debug, Clone)]
pub struct G3CheckResult {
    /// Rule identifier (e.g., "RS-CLIPPY-02").
    id: String,
    /// How severe this finding is.
    severity: G3Severity,
    /// Short title for summary display.
    title: String,
    /// Full description with context and remediation.
    message: String,
    /// Repo-relative file path this finding relates to.
    file: Option<String>,
    /// 1-based line number in the file.
    line: Option<usize>,
    /// Whether this result is inventory (hidden unless `--inventory`).
    inventory: bool,
}

impl G3CheckResult {
    /// Create a new check result.
    #[must_use]
    pub const fn new(
        id: String,
        severity: G3Severity,
        title: String,
        message: String,
        file: Option<String>,
        line: Option<usize>,
    ) -> Self {
        Self {
            id,
            severity,
            title,
            message,
            file,
            line,
            inventory: false,
        }
    }

    /// Mark this result as inventory (hidden by default). Consumes and returns self.
    #[must_use]
    pub const fn into_inventory(mut self) -> Self {
        self.inventory = true;
        self
    }

    /// Rule identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Severity level.
    #[must_use]
    pub const fn severity(&self) -> G3Severity {
        self.severity
    }

    /// Short title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Full message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// File path if available.
    #[must_use]
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Line number if available.
    #[must_use]
    pub const fn line(&self) -> Option<usize> {
        self.line
    }

    /// Whether this is an inventory item.
    #[must_use]
    pub const fn inventory(&self) -> bool {
        self.inventory
    }
}
