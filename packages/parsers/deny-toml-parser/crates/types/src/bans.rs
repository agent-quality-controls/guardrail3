#![allow(
    clippy::module_name_repetitions,
    reason = "deny.toml schema mirror: types in this module intentionally repeat the deny.toml table name they model"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Dependency ban settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BansConfig {
    /// Action for multiple versions of the same crate.
    pub multiple_versions: Option<String>,
    /// Whether dev-dependencies are included when checking multiple versions.
    pub multiple_versions_include_dev: Option<bool>,
    /// Action for wildcard dependencies.
    pub wildcards: Option<String>,
    /// Whether to allow wildcard versions for path dependencies.
    pub allow_wildcard_paths: Option<bool>,
    /// Highlight mode for duplicate versions: `"all"`, `"lowest-version"`, `"simplest-path"`.
    pub highlight: Option<String>,
    /// Default-feature lint level for workspace crates.
    pub workspace_default_features: Option<String>,
    /// Default-feature lint level for external crates.
    pub external_default_features: Option<String>,
    /// Whether workspace members are automatically allowlisted.
    pub allow_workspace: Option<bool>,
    /// Crates to explicitly deny.
    #[serde(default)]
    pub deny: Vec<BanDenyEntry>,
    /// Crates to explicitly allow.
    #[serde(default)]
    pub allow: Vec<BanAllowEntry>,
    /// Specific crate versions to skip in duplicate checks.
    #[serde(default)]
    pub skip: Vec<BanSkipEntry>,
    /// Crate trees to skip in duplicate checks.
    #[serde(default)]
    pub skip_tree: Vec<BanSkipTreeEntry>,
    /// Feature-level ban configuration.
    #[serde(default)]
    pub features: Vec<BanFeatureEntry>,
    /// Workspace dependency policy.
    pub workspace_dependencies: Option<BanWorkspaceDependenciesConfig>,
    /// Build-time crate policy.
    pub build: Option<BanBuildConfig>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// --- Ban deny entries ---

/// An entry in `[bans].deny`: either a bare crate name string
/// or a detailed table with name, version, wrappers, and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanDenyEntry {
    /// Bare crate name string, e.g. `"openssl"`.
    Simple(String),
    /// Detailed entry: `{ name = "openssl", wrappers = [], reason = "..." }`.
    Detailed(BanDenyDetail),
}

/// Detailed ban deny entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanDenyDetail {
    /// The crate name.
    pub name: Option<String>,
    /// Alternative crate identifier (deprecated; prefer `name`).
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Version requirement to match.
    pub version: Option<String>,
    /// Crates that are allowed to transitively depend on the banned crate.
    #[serde(default)]
    pub wrappers: Vec<String>,
    /// Whether this crate should deny multiple versions of itself.
    pub deny_multiple_versions: Option<bool>,
    /// Why this crate is banned.
    pub reason: Option<String>,
    /// Preferred replacement crate.
    pub use_instead: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// --- Ban allow entries ---

/// An entry in `[bans].allow`: either a bare crate name string
/// or a detailed table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanAllowEntry {
    /// Bare crate name string, e.g. `"serde"`.
    Simple(String),
    /// Detailed entry: `{ name = "serde", version = "1.0" }`.
    Detailed(BanAllowDetail),
}

/// Detailed ban allow entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanAllowDetail {
    /// The crate name.
    pub name: Option<String>,
    /// Alternative crate identifier.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Version requirement to match.
    pub version: Option<String>,
    /// Why this crate is allowed.
    pub reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// --- Ban skip entries ---

/// An entry in `[bans].skip`: either a bare crate name string
/// or a detailed table with name, version, and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanSkipEntry {
    /// Bare crate name string, e.g. `"windows-sys"`.
    Simple(String),
    /// Detailed entry: `{ name = "windows-sys", version = "=0.48", reason = "..." }`.
    Detailed(BanSkipDetail),
}

/// Detailed ban skip entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanSkipDetail {
    /// The crate name.
    pub name: Option<String>,
    /// Alternative crate identifier.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Version requirement to skip.
    pub version: Option<String>,
    /// Why this skip is necessary.
    pub reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// --- Ban feature entries ---

/// Feature-level ban configuration for a specific crate.
///
/// Appears as `[[bans.features]]` in deny.toml.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanFeatureEntry {
    /// The crate name this feature rule applies to.
    pub name: Option<String>,
    /// Alternative crate identifier.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// Features to deny.
    #[serde(default)]
    pub deny: Vec<String>,
    /// Features to allow (implicitly denies everything else).
    #[serde(default)]
    pub allow: Vec<String>,
    /// Why this feature rule is needed.
    pub reason: Option<String>,
    /// Whether to treat the feature set as exact.
    pub exact: Option<bool>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// An entry in `[bans].skip-tree`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanSkipTreeEntry {
    /// Bare package-spec string.
    Simple(String),
    /// Detailed skip-tree entry.
    Detailed(BanSkipTreeDetail),
}

/// Detailed skip-tree entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanSkipTreeDetail {
    /// Deprecated crate name field.
    pub name: Option<String>,
    /// Package spec this skip-tree applies to.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// How deep the skip should extend.
    pub depth: Option<u64>,
    /// Why this skip-tree is necessary.
    pub reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// `[bans.workspace-dependencies]` configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanWorkspaceDependenciesConfig {
    /// Handling for duplicate workspace dependencies.
    pub duplicates: Option<String>,
    /// Whether path dependencies are included.
    pub include_path_dependencies: Option<bool>,
    /// Handling for unused workspace dependencies.
    pub unused: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// `[bans.build]` configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanBuildConfig {
    /// Crates allowed to have build scripts.
    #[serde(default)]
    pub allow_build_scripts: Vec<BanBuildAllowBuildScriptEntry>,
    /// Handling for native executables.
    pub executables: Option<String>,
    /// Handling for interpreted scripts.
    pub interpreted: Option<String>,
    /// Extensions to scan for.
    #[serde(default)]
    pub script_extensions: Vec<String>,
    /// Whether builtin glob patterns are enabled.
    pub enable_builtin_globs: Option<bool>,
    /// Whether dependencies of compile-time crates are scanned.
    pub include_dependencies: Option<bool>,
    /// Whether workspace crates are scanned.
    pub include_workspace: Option<bool>,
    /// Whether archives are counted as native code.
    pub include_archives: Option<bool>,
    /// Per-crate build scan bypasses.
    #[serde(default)]
    pub bypass: Vec<BanBuildBypassEntry>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A package-spec entry under `allow-build-scripts`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanBuildAllowBuildScriptEntry {
    /// Bare package-spec string.
    Simple(String),
    /// Detailed package-spec table.
    Detailed(BanBuildAllowBuildScriptDetail),
}

/// Detailed package-spec table under `allow-build-scripts`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanBuildAllowBuildScriptDetail {
    /// Deprecated crate name field.
    pub name: Option<String>,
    /// Package spec this entry applies to.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One `[[bans.build.bypass]]` entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanBuildBypassEntry {
    /// Deprecated crate name field.
    pub name: Option<String>,
    /// Package spec this bypass applies to.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// Optional build-script checksum.
    pub build_script: Option<String>,
    /// Required features that gate the bypass.
    #[serde(default)]
    pub required_features: Vec<String>,
    /// Globs to bypass scanning for.
    #[serde(default)]
    pub allow_globs: Vec<String>,
    /// Individual files to bypass.
    #[serde(default)]
    pub allow: Vec<BanBuildBypassAllowEntry>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One file-level bypass entry under `bans.build.bypass.allow`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanBuildBypassAllowEntry {
    /// Path relative to the crate root.
    pub path: String,
    /// Optional SHA-256 checksum.
    pub checksum: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
