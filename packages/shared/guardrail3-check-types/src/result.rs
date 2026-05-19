use crate::G3Severity;

/// A single finding from a guardrail3 check.
///
/// Every check package returns `Vec<G3CheckResult>`. The orchestrator
/// collects results from all packages and renders the report.
#[derive(Debug, Clone)]
pub struct G3CheckResult {
    /// Rule identifier (e.g., "RS-CLIPPY-CONFIG-01").
    id: String,
    /// How severe this finding is.
    severity: G3Severity,
    /// Short title for summary display.
    title: String,
    /// Full description with context and remediation.
    message: String,
    /// Repo-relative file path this finding relates to.
    file: Option<String>,
    /// Stable waiver subject. Defaults to the display file or `-`.
    subject: String,
    /// Stable waiver selector. Defaults to `line:<n>` or a title slug.
    selector: String,
    /// 1-based line number in the file.
    line: Option<usize>,
    /// Reason from the matching central waiver, if any.
    waiver_reason: Option<String>,
    /// Whether this result is inventory (hidden unless `--inventory`).
    inventory: bool,
}

impl G3CheckResult {
    /// Create a new check result.
    #[must_use]
    pub fn new(
        id: String,
        severity: G3Severity,
        title: String,
        message: String,
        file: Option<String>,
        line: Option<usize>,
    ) -> Self {
        let subject = file.clone().unwrap_or_else(|| "-".to_owned());
        let selector = default_selector(&title, line);
        Self {
            id,
            severity,
            title,
            message,
            file,
            subject,
            selector,
            line,
            waiver_reason: None,
            inventory: false,
        }
    }

    /// Mark this result as inventory (hidden by default). Consumes and returns self.
    #[must_use]
    pub const fn into_inventory(mut self) -> Self {
        self.inventory = true;
        self
    }

    /// Override the default waiver subject.
    #[must_use]
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    /// Override the default waiver selector.
    #[must_use]
    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = selector.into();
        self
    }

    /// Apply a matching central waiver to this finding.
    pub fn apply_waiver(&mut self, reason: &str) {
        if self.severity == G3Severity::Error {
            self.severity = G3Severity::Warn;
        }
        self.waiver_reason = Some(reason.to_owned());
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

    /// Returns title and message string slices as a tuple.
    #[must_use]
    pub fn text(&self) -> (&str, &str) {
        (self.title.as_str(), self.message.as_str())
    }

    /// Short title.
    #[must_use]
    pub fn title(&self) -> &str {
        self.text().0
    }

    /// Full message.
    #[must_use]
    pub fn message(&self) -> &str {
        let (_, body) = self.text();
        body
    }

    /// File path if available.
    #[must_use]
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Waiver subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    /// Waiver selector.
    #[must_use]
    pub fn selector(&self) -> &str {
        self.selector.as_str()
    }

    /// Rule, subject, and selector used by central waiver matching.
    #[must_use]
    pub fn waiver_key(&self) -> (&str, &str, &str) {
        (self.id(), self.subject(), self.selector())
    }

    /// Configured waiver reason, if this finding matched a waiver.
    #[must_use]
    pub fn waiver_reason(&self) -> Option<&str> {
        self.waiver_reason.as_deref()
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

/// Builds the default selector for legacy result constructors.
fn default_selector(title: &str, line: Option<usize>) -> String {
    if let Some(line) = line {
        return format!("line:{line}");
    }
    format!("title:{}", slug(title))
}

/// Converts display titles into deterministic selector fragments.
fn slug(input: &str) -> String {
    let mut out = String::new();
    let mut previous_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }
    out.trim_matches('-').to_owned()
}
