use guardrail3_rs_toml_parser::RustProfile;

/// Normalized section kind for one dependency entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsDepsDependencySection {
    /// `[dependencies]` or `[target.*.dependencies]`
    Dependencies,
    /// `[build-dependencies]` or `[target.*.build-dependencies]`
    BuildDependencies,
    /// `[dev-dependencies]` or `[target.*.dev-dependencies]`
    DevDependencies,
}

/// One normalized external dependency entry ready for config checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsDepsResolvedDependency {
    /// Canonical package identity used for allowlist and cap checks.
    pub package_name: String,
    /// Dependency section that owns this entry.
    pub section: G3RsDepsDependencySection,
    /// Exact Cargo table label where the dependency came from.
    pub table_label: String,
}

/// Input contract for extracted dependency config checks.
///
/// Ingestion owns crawl selection, manifest parsing, workspace dependency
/// resolution, and normalization into external dependency facts.
#[derive(Debug, Clone)]
pub struct G3RsDepsConfigChecksInput {
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub crate_cargo_rel_path: String,
    /// Resolved crate identity for messages.
    pub crate_name: String,
    /// Workspace policy profile from `guardrail3-rs.toml`, if present.
    pub profile: Option<RustProfile>,
    /// Whether `allowed_deps` was explicitly present in `guardrail3-rs.toml`.
    pub allowlist_present: bool,
    /// Workspace allowlist policy for this crate.
    pub allowed_deps: Vec<String>,
    /// Normalized external dependency entries owned by config checks.
    pub dependencies: Vec<G3RsDepsResolvedDependency>,
}

/// Placeholder input contract for future deps source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDepsSourceChecksInput;

/// Placeholder input contract for future deps file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsDepsFileTreeChecksInput;
