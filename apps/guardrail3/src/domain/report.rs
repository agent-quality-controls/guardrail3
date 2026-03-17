#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub inventory: bool,
}

impl CheckResult {
    /// Mark this result as inventory (hidden unless `--inventory` flag is set).
    /// Only use for passing/confirmation Info results — never for problems or audit trails.
    #[must_use]
    pub const fn as_inventory(mut self) -> Self {
        self.inventory = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub results: Vec<CheckResult>,
}

#[derive(Debug)]
pub struct Report {
    pub project_path: String,
    pub stacks: Vec<String>,
    pub sections: Vec<Section>,
}

/// Controls which validation domains are active.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: domain flags are independent toggles, not a state machine
pub struct ValidateDomains {
    pub code: bool,
    pub architecture: bool,
    pub release: bool,
    pub tests: bool,
}

/// Resolved check categories for Rust validation.
/// Built by merging guardrail3.toml [rust.checks] with CLI flags.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: check category flags are independent toggles, not a state machine
pub struct RustCheckCategories {
    pub architecture: bool,
    pub garde: bool,
    pub tests: bool,
    pub release: bool,
}

impl Default for RustCheckCategories {
    fn default() -> Self {
        Self {
            architecture: true,
            garde: true,
            tests: true,
            release: true,
        }
    }
}

/// Resolved check categories for TypeScript validation.
/// Built by merging guardrail3.toml [typescript.checks] with CLI flags.
#[derive(Debug, Clone)]
pub struct TsCheckCategories {
    pub architecture: bool,
    pub tests: bool,
}

impl Default for TsCheckCategories {
    fn default() -> Self {
        Self {
            architecture: true,
            tests: true,
        }
    }
}

impl Report {
    pub const fn new(project_path: String, stacks: Vec<String>) -> Self {
        Self {
            project_path,
            stacks,
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    fn count_by_severity(&self, severity: Severity) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity == severity)
            .count()
    }

    pub fn inventory_count(&self) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.inventory)
            .count()
    }
}

/// Generate a severity-count method on `Report` to avoid structural duplication.
macro_rules! severity_counter {
    ($name:ident, $variant:ident) => {
        impl Report {
            pub fn $name(&self) -> usize {
                self.count_by_severity(Severity::$variant)
            }
        }
    };
}

severity_counter!(error_count, Error);
severity_counter!(warn_count, Warn);
severity_counter!(info_count, Info);
