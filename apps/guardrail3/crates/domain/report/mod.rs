use guardrail3_validation_model::RustValidateFamily;

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
    #[must_use]
    pub fn new(id: String, severity: Severity, title: String, message: String) -> Self {
        Self {
            id,
            severity,
            title,
            message,
            file: None,
            line: None,
            inventory: false,
        }
    }

    #[must_use]
    pub fn from_parts(
        id: String,
        severity: Severity,
        title: String,
        message: String,
        file: Option<String>,
        line: Option<usize>,
        inventory: bool,
    ) -> Self {
        Self {
            id,
            severity,
            title,
            message,
            file,
            line,
            inventory,
        }
    }

    #[must_use]
    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    #[must_use]
    pub const fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    #[must_use]
    pub fn with_optional_file(mut self, file: Option<String>) -> Self {
        self.file = file;
        self
    }

    #[must_use]
    pub const fn with_optional_line(mut self, line: Option<usize>) -> Self {
        self.line = line;
        self
    }

    #[must_use]
    pub fn with_optional_location(self, file: Option<String>, line: Option<usize>) -> Self {
        self.with_optional_file(file).with_optional_line(line)
    }

    #[must_use]
    pub const fn with_inventory(mut self, inventory: bool) -> Self {
        self.inventory = inventory;
        self
    }

    /// Mark this result as inventory (hidden unless `--inventory` flag is set).
    /// Only use for passing/confirmation Info results — never for problems or audit trails.
    #[must_use]
    pub const fn as_inventory(mut self) -> Self {
        self.inventory = true;
        self
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    #[must_use]
    pub const fn line(&self) -> Option<usize> {
        self.line
    }

    #[must_use]
    pub const fn inventory(&self) -> bool {
        self.inventory
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    name: String,
    results: Vec<CheckResult>,
}

impl Section {
    #[must_use]
    pub fn new(name: String, results: Vec<CheckResult>) -> Self {
        Self { name, results }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn results(&self) -> &[CheckResult] {
        &self.results
    }
}

#[derive(Debug)]
pub struct Report {
    project_path: String,
    stacks: Vec<String>,
    sections: Vec<Section>,
}

/// Controls which validation domains are active.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: domain flags are independent toggles, not a state machine
pub struct ValidateDomains {
    code: bool,
    topology: bool,
    release: bool,
    tests: bool,
}

impl ValidateDomains {
    #[must_use]
    pub const fn new(code: bool, topology: bool, release: bool, tests: bool) -> Self {
        Self {
            code,
            topology,
            release,
            tests,
        }
    }

    #[must_use]
    pub const fn code(&self) -> bool {
        self.code
    }

    #[must_use]
    pub const fn topology(&self) -> bool {
        self.topology
    }

    #[must_use]
    pub const fn release(&self) -> bool {
        self.release
    }

    #[must_use]
    pub const fn tests(&self) -> bool {
        self.tests
    }
}

/// Resolved check categories for Rust validation.
/// Built by merging guardrail3.toml [rust.checks] with CLI flags.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: check category flags are independent toggles, not a state machine
pub struct RustCheckCategories {
    topology: bool,
    garde: bool,
    hooks: bool,
    tests: bool,
    release: bool,
}

impl RustCheckCategories {
    #[must_use]
    pub const fn new(
        topology: bool,
        garde: bool,
        hooks: bool,
        tests: bool,
        release: bool,
    ) -> Self {
        Self {
            topology,
            garde,
            hooks,
            tests,
            release,
        }
    }

    #[must_use]
    pub const fn topology(&self) -> bool {
        self.topology
    }

    #[must_use]
    pub const fn garde(&self) -> bool {
        self.garde
    }

    #[must_use]
    pub const fn hooks(&self) -> bool {
        self.hooks
    }

    #[must_use]
    pub const fn tests(&self) -> bool {
        self.tests
    }

    #[must_use]
    pub const fn release(&self) -> bool {
        self.release
    }
}

impl Default for RustCheckCategories {
    fn default() -> Self {
        Self {
            topology: true,
            garde: true,
            hooks: true,
            tests: true,
            release: true,
        }
    }
}

#[must_use]
pub const fn rust_validate_family_cli_name(family: RustValidateFamily) -> &'static str {
    match family {
        RustValidateFamily::Topology => "topology",
        RustValidateFamily::Arch => "arch",
        RustValidateFamily::Fmt => "fmt",
        RustValidateFamily::Toolchain => "toolchain",
        RustValidateFamily::Clippy => "clippy",
        RustValidateFamily::Deny => "deny",
        RustValidateFamily::Cargo => "cargo",
        RustValidateFamily::Code => "code",
        RustValidateFamily::Hexarch => "hexarch",

        RustValidateFamily::Deps => "deps",
        RustValidateFamily::Garde => "garde",
        RustValidateFamily::Test => "test",
        RustValidateFamily::Release => "release",
        RustValidateFamily::HooksShared => "hooks-shared",
        RustValidateFamily::HooksRs => "hooks-rs",
    }
}

#[must_use]
pub const fn rust_validate_family_config_key(family: RustValidateFamily) -> &'static str {
    match family {
        RustValidateFamily::Topology => "topology",
        RustValidateFamily::Arch => "arch",
        RustValidateFamily::Fmt => "fmt",
        RustValidateFamily::Toolchain => "toolchain",
        RustValidateFamily::Clippy => "clippy",
        RustValidateFamily::Deny => "deny",
        RustValidateFamily::Cargo => "cargo",
        RustValidateFamily::Code => "code",
        RustValidateFamily::Hexarch => "hexarch",

        RustValidateFamily::Deps => "deps",
        RustValidateFamily::Garde => "garde",
        RustValidateFamily::Test => "test",
        RustValidateFamily::Release => "release",
        RustValidateFamily::HooksShared => "hooks_shared",
        RustValidateFamily::HooksRs => "hooks_rs",
    }
}

#[must_use]
pub const fn rust_validate_family_section_name(family: RustValidateFamily) -> &'static str {
    rust_validate_family_cli_name(family)
}

/// Resolved check categories for TypeScript validation.
/// Built by merging guardrail3.toml [typescript.checks] with CLI flags.
#[derive(Debug, Clone)]
pub struct TsCheckCategories {
    topology: bool,
    content: bool,
    tests: bool,
}

impl TsCheckCategories {
    #[must_use]
    pub const fn new(topology: bool, content: bool, tests: bool) -> Self {
        Self {
            topology,
            content,
            tests,
        }
    }

    #[must_use]
    pub const fn topology(&self) -> bool {
        self.topology
    }

    #[must_use]
    pub const fn content(&self) -> bool {
        self.content
    }

    #[must_use]
    pub const fn tests(&self) -> bool {
        self.tests
    }
}

impl Default for TsCheckCategories {
    fn default() -> Self {
        Self::new(true, true, true)
    }
}

/// TypeScript app type — determines which check categories apply by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsAppType {
    /// HTTP server, API backend — hexarch, route wrappers, full `ESLint` strict
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
                topology: true,
                content: false,
                tests: true,
            },
            Self::Content => TsCheckCategories {
                topology: false,
                content: true,
                tests: true,
            },
            Self::Library => TsCheckCategories {
                topology: false,
                content: false,
                tests: true,
            },
        }
    }
}

/// Resolved per-app context for TypeScript validation.
#[derive(Debug, Clone)]
pub struct TsAppContext {
    name: String,
    path: std::path::PathBuf,
    app_type: TsAppType,
    categories: TsCheckCategories,
}

impl TsAppContext {
    #[must_use]
    pub fn new(
        name: String,
        path: std::path::PathBuf,
        app_type: TsAppType,
        categories: TsCheckCategories,
    ) -> Self {
        Self {
            name,
            path,
            app_type,
            categories,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    #[must_use]
    pub const fn app_type(&self) -> TsAppType {
        self.app_type
    }

    #[must_use]
    pub const fn categories(&self) -> &TsCheckCategories {
        &self.categories
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

    #[must_use]
    pub fn project_path(&self) -> &str {
        &self.project_path
    }

    #[must_use]
    pub fn stacks(&self) -> &[String] {
        &self.stacks
    }

    #[must_use]
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    fn count_by_severity(&self, severity: Severity) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity() == severity)
            .count()
    }

    pub fn inventory_count(&self) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.inventory())
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

#[cfg(test)]
mod report_tests;
