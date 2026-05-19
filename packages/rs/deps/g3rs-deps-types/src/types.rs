use g3rs_toml_parser::types::RustProfile;
use serde::Serialize;

/// Scope of one deps config input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum G3RsDepsConfigInputScope {
    /// One workspace-scoped tooling snapshot.
    WorkspaceTooling,
    /// One crate-scoped dependency policy input.
    CratePolicy,
}

/// Normalized section kind for one dependency entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3RsDepsDependencySection {
    /// `[dependencies]` or `[target.*.dependencies]`
    Dependencies,
    /// `[build-dependencies]` or `[target.*.build-dependencies]`
    BuildDependencies,
    /// `[dev-dependencies]` or `[target.*.dev-dependencies]`
    DevDependencies,
}

/// One normalized external dependency entry ready for config checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
#[derive(Debug, Clone, Serialize)]
pub struct G3RsDepsConfigChecksInput {
    /// Whether this input is workspace tooling or crate policy.
    pub scope: G3RsDepsConfigInputScope,
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
    /// Tool names discovered on PATH for the pointed workspace process environment.
    pub installed_tools: Vec<String>,
}

/// Placeholder input contract for future deps source checks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct G3RsDepsSourceChecksInput;

/// Input contract for deps file-tree checks at one pointed workspace root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsDepsFileTreeChecksInput {
    /// Workspace policy profile from `guardrail3-rs.toml`, when parseable.
    pub profile: Option<RustProfile>,
    /// Repo-relative root `Cargo.lock` path owned by this workspace.
    pub cargo_lock_rel_path: String,
    /// Whether the root `Cargo.lock` exists.
    pub cargo_lock_exists: bool,
    /// Whether a relevant root `.gitignore` masks `Cargo.lock`.
    pub cargo_lock_ignored: bool,
    /// `.gitignore` path responsible for masking `Cargo.lock`, when any.
    pub gitignore_rel_path: Option<String>,
}
