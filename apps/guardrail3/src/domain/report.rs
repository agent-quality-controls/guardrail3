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
    pub content: bool,
    pub tests: bool,
}

impl Default for TsCheckCategories {
    fn default() -> Self {
        Self {
            architecture: true,
            content: true,
            tests: true,
        }
    }
}

/// TypeScript app type — determines which check categories apply by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsAppType {
    /// HTTP server, API backend — hex arch, route wrappers, full `ESLint` strict
    Service,
    /// Marketing site, blog, docs — content checks, SEO, accessibility, static gen
    Content,
    /// Shared package, no I/O — dependency restrictions
    Library,
}

impl TsAppType {
    /// Parse from config string (case-insensitive), defaulting to Service for unknown values.
    pub fn from_str_or_default(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "content" => Self::Content,
            "library" => Self::Library,
            _ => Self::Service,
        }
    }

    /// Default check categories for this app type.
    #[must_use]
    pub const fn default_categories(self) -> TsCheckCategories {
        match self {
            Self::Service => TsCheckCategories {
                architecture: true,
                content: false,
                tests: true,
            },
            Self::Content => TsCheckCategories {
                architecture: false,
                content: true,
                tests: true,
            },
            Self::Library => TsCheckCategories {
                architecture: false,
                content: false,
                tests: true,
            },
        }
    }
}

/// Resolved per-app context for TypeScript validation.
#[derive(Debug, Clone)]
pub struct TsAppContext {
    pub name: String,
    pub path: std::path::PathBuf,
    pub app_type: TsAppType,
    pub categories: TsCheckCategories,
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
