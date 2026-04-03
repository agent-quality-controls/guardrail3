use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::Error;

// =============================================================================
// Top-level config
// =============================================================================

/// Parsed representation of a `deny.toml` configuration file.
///
/// All five top-level sections (`graph`, `advisories`, `bans`, `licenses`,
/// `sources`) are mapped to typed fields. An optional `output` section
/// is also captured. Unknown keys are captured in [`extra`](Self::extra)
/// for forward compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct DenyConfig {
    /// Dependency graph configuration.
    pub graph: Option<GraphConfig>,
    /// Security advisory checking settings.
    pub advisories: Option<AdvisoriesConfig>,
    /// Dependency ban settings.
    pub bans: Option<BansConfig>,
    /// License checking settings.
    pub licenses: Option<LicensesConfig>,
    /// Source restrictions.
    pub sources: Option<SourcesConfig>,
    /// Output formatting configuration.
    pub output: Option<OutputConfig>,

    /// Unknown top-level keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// =============================================================================
// [graph]
// =============================================================================

/// Settings for the dependency graph resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GraphConfig {
    /// Enable all features when resolving the dependency graph.
    pub all_features: Option<bool>,
    /// Disable default features when resolving the dependency graph.
    pub no_default_features: Option<bool>,
    /// Target triples to check.
    #[serde(default)]
    pub targets: Vec<Value>,
    /// Crates to exclude from all checks.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// =============================================================================
// [advisories]
// =============================================================================

/// Security advisory checking settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdvisoriesConfig {
    /// Action for unmaintained crates: `"deny"`, `"warn"`, `"allow"`, or `"workspace"`.
    pub unmaintained: Option<String>,
    /// Action for yanked crates: `"deny"`, `"warn"`, `"allow"`.
    pub yanked: Option<String>,
    /// Deprecated: action for vulnerability advisories.
    pub vulnerability: Option<String>,
    /// Deprecated: action for notice advisories.
    pub notice: Option<String>,
    /// Deprecated: action for unsound advisories.
    pub unsound: Option<String>,
    /// Advisory IDs to ignore.
    #[serde(default)]
    pub ignore: Vec<AdvisoryIgnoreEntry>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// An entry in `[advisories].ignore`: either a bare advisory ID string
/// or a detailed table with an `id` and optional `reason`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdvisoryIgnoreEntry {
    /// Bare advisory ID string, e.g. `"RUSTSEC-2024-0001"`.
    Simple(String),
    /// Detailed entry: `{ id = "RUSTSEC-2024-0001", reason = "..." }`.
    Detailed(AdvisoryIgnoreDetail),
}

impl AdvisoryIgnoreEntry {
    /// Returns the advisory ID regardless of entry format.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Simple(id) => id,
            Self::Detailed(detail) => detail.id(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed advisory ignore entry with an ID and optional reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdvisoryIgnoreDetail {
    /// The advisory ID (e.g. `"RUSTSEC-2024-0001"`).
    id: String,
    /// Why this advisory is being ignored.
    reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl AdvisoryIgnoreDetail {
    /// The advisory ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Why this advisory is being ignored, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// =============================================================================
// [bans]
// =============================================================================

/// Dependency ban settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BansConfig {
    /// Action for multiple versions of the same crate.
    pub multiple_versions: Option<String>,
    /// Action for wildcard dependencies.
    pub wildcards: Option<String>,
    /// Whether to allow wildcard versions for path dependencies.
    pub allow_wildcard_paths: Option<bool>,
    /// Highlight mode for duplicate versions: `"all"`, `"lowest-version"`, `"simplest-path"`.
    pub highlight: Option<String>,
    /// Crates to explicitly deny.
    #[serde(default)]
    pub deny: Vec<BanDenyEntry>,
    /// Crates to explicitly allow.
    #[serde(default)]
    pub allow: Vec<BanAllowEntry>,
    /// Specific crate versions to skip in duplicate checks.
    #[serde(default)]
    pub skip: Vec<BanSkipEntry>,
    /// Feature-level ban configuration.
    #[serde(default)]
    pub features: Vec<BanFeatureEntry>,
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

impl BanDenyEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed ban deny entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanDenyDetail {
    /// The crate name.
    name: Option<String>,
    /// Alternative crate identifier (deprecated; prefer `name`).
    #[serde(rename = "crate")]
    crate_name: Option<String>,
    /// Version requirement to match.
    version: Option<String>,
    /// Crates that are allowed to transitively depend on the banned crate.
    #[serde(default)]
    wrappers: Vec<String>,
    /// Why this crate is banned.
    reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanDenyDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Alternative crate identifier (deprecated; prefer `name`).
    #[must_use]
    pub fn crate_name(&self) -> Option<&str> {
        self.crate_name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Wrapper crates allowed to depend on this banned crate.
    #[must_use]
    pub fn wrappers(&self) -> &[String] {
        &self.wrappers
    }

    /// Why this crate is banned, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
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

impl BanAllowEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }
}

/// Detailed ban allow entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanAllowDetail {
    /// The crate name.
    name: Option<String>,
    /// Version requirement to match.
    version: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanAllowDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
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

impl BanSkipEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed ban skip entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanSkipDetail {
    /// The crate name.
    name: Option<String>,
    /// Version requirement to skip.
    version: Option<String>,
    /// Why this skip is necessary.
    reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanSkipDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Why this skip is necessary, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// --- Ban feature entries ---

/// Feature-level ban configuration for a specific crate.
///
/// Appears as `[[bans.features]]` in deny.toml.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanFeatureEntry {
    /// The crate name this feature rule applies to.
    name: Option<String>,
    /// Features to deny.
    #[serde(default)]
    deny: Vec<String>,
    /// Features to allow (implicitly denies everything else).
    #[serde(default)]
    allow: Vec<String>,
    /// Whether to treat the feature set as exact.
    exact: Option<bool>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanFeatureEntry {
    /// The crate name this feature rule applies to, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Features to deny.
    #[must_use]
    pub fn deny(&self) -> &[String] {
        &self.deny
    }

    /// Features to allow (implicitly denies everything else).
    #[must_use]
    pub fn allow(&self) -> &[String] {
        &self.allow
    }

    /// Whether to treat the feature set as exact.
    #[must_use]
    pub const fn exact(&self) -> Option<bool> {
        self.exact
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// =============================================================================
// [licenses]
// =============================================================================

/// License checking settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesConfig {
    /// Allowed license SPDX identifiers.
    #[serde(default)]
    pub allow: Vec<String>,
    /// Minimum confidence threshold for license detection (0.0 to 1.0).
    pub confidence_threshold: Option<f64>,
    /// Per-crate license exceptions.
    #[serde(default)]
    pub exceptions: Vec<LicenseException>,
    /// Configuration for private/unpublished crates.
    pub private: Option<LicensesPrivateConfig>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A per-crate license exception.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicenseException {
    /// The crate name.
    name: String,
    /// Allowed licenses for this specific crate.
    #[serde(default)]
    allow: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl LicenseException {
    /// The crate name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Allowed licenses for this specific crate.
    #[must_use]
    pub fn allow(&self) -> &[String] {
        &self.allow
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

/// Configuration for private/unpublished crates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesPrivateConfig {
    /// Whether to ignore license requirements for private crates.
    ignore: Option<bool>,
    /// Registries whose crates are considered private.
    #[serde(default)]
    registries: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl LicensesPrivateConfig {
    /// Whether to ignore license requirements for private crates.
    #[must_use]
    pub const fn ignore(&self) -> Option<bool> {
        self.ignore
    }

    /// Registries whose crates are considered private.
    #[must_use]
    pub fn registries(&self) -> &[String] {
        &self.registries
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// =============================================================================
// [sources]
// =============================================================================

/// Source restriction settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SourcesConfig {
    /// Action for unknown registries: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_registry: Option<String>,
    /// Action for unknown git sources: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_git: Option<String>,
    /// Allowed registries.
    #[serde(default)]
    pub allow_registry: Vec<String>,
    /// Allowed git sources.
    #[serde(default)]
    pub allow_git: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// =============================================================================
// [output]
// =============================================================================

/// Output formatting configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OutputConfig {
    /// Feature depth for dependency graph output.
    pub feature_depth: Option<u32>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// =============================================================================
// Constructors
// =============================================================================

impl DenyConfig {
    /// Read and parse a deny.toml file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] on read failure, [`Error::Toml`] on parse failure.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let content = crate::fs::read_to_string(path)?;
        content.parse()
    }
}

impl std::str::FromStr for DenyConfig {
    type Err = Error;

    #[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized deny.toml parser — toml::from_str is its core purpose
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse helper that panics with context on failure.
    fn parse(input: &str) -> DenyConfig {
        input.parse::<DenyConfig>().expect("should parse valid deny.toml")
    }

    #[test]
    fn empty_string_yields_empty_config() {
        let cfg = parse("");

        assert_eq!(cfg.graph, None, "graph should be None");
        assert_eq!(cfg.advisories, None, "advisories should be None");
        assert_eq!(cfg.bans, None, "bans should be None");
        assert_eq!(cfg.licenses, None, "licenses should be None");
        assert_eq!(cfg.sources, None, "sources should be None");
        assert_eq!(cfg.output, None, "output should be None");
        assert!(cfg.extra.is_empty(), "extra should be empty");
    }

    #[test]
    fn graph_section_parses() {
        let cfg = parse(
            r#"
[graph]
all-features = true
no-default-features = false
targets = ["x86_64-unknown-linux-gnu"]
exclude = ["some-crate"]
"#,
        );

        let graph = cfg.graph.expect("graph should be present");
        assert_eq!(graph.all_features, Some(true), "all_features mismatch");
        assert_eq!(graph.no_default_features, Some(false), "no_default_features mismatch");
        assert_eq!(graph.targets.len(), 1, "should have 1 target");
        assert_eq!(graph.exclude.len(), 1, "should have 1 exclude");
        assert_eq!(graph.exclude[0], "some-crate", "exclude value mismatch");
        assert!(graph.extra.is_empty(), "graph extra should be empty");
    }

    #[test]
    fn advisories_section_parses() {
        let cfg = parse(
            r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []
"#,
        );

        let adv = cfg.advisories.expect("advisories should be present");
        assert_eq!(adv.unmaintained, Some("workspace".into()), "unmaintained mismatch");
        assert_eq!(adv.yanked, Some("warn".into()), "yanked mismatch");
        assert!(adv.ignore.is_empty(), "ignore should be empty");
        assert!(adv.extra.is_empty(), "advisories extra should be empty");
    }

    #[test]
    fn advisories_deprecated_fields_parse() {
        let cfg = parse(
            r#"
[advisories]
vulnerability = "deny"
notice = "warn"
unsound = "deny"
"#,
        );

        let adv = cfg.advisories.expect("advisories should be present");
        assert_eq!(adv.vulnerability, Some("deny".into()), "vulnerability mismatch");
        assert_eq!(adv.notice, Some("warn".into()), "notice mismatch");
        assert_eq!(adv.unsound, Some("deny".into()), "unsound mismatch");
    }

    #[test]
    fn advisory_ignore_entries() {
        let cfg = parse(
            r#"
[advisories]
ignore = [
    "RUSTSEC-2024-0001",
    { id = "RUSTSEC-2024-0002", reason = "Not applicable" },
]
"#,
        );

        let adv = cfg.advisories.expect("advisories should be present");
        assert_eq!(adv.ignore.len(), 2, "should have 2 ignore entries");
        assert_eq!(adv.ignore[0].id(), "RUSTSEC-2024-0001", "first ignore ID");
        assert_eq!(adv.ignore[0].reason(), None, "simple entry has no reason");
        assert_eq!(adv.ignore[1].id(), "RUSTSEC-2024-0002", "second ignore ID");
        assert_eq!(
            adv.ignore[1].reason(),
            Some("Not applicable"),
            "detailed entry should have reason",
        );
    }

    #[test]
    fn bans_simple_deny_entries() {
        let cfg = parse(
            r#"
[bans]
multiple-versions = "deny"
deny = ["openssl", "chrono"]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.multiple_versions, Some("deny".into()), "multiple_versions mismatch");
        assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");
        assert_eq!(bans.deny[0].name(), Some("openssl"), "first deny name");
        assert_eq!(bans.deny[1].name(), Some("chrono"), "second deny name");
    }

    #[test]
    fn bans_detailed_deny_entries() {
        let cfg = parse(
            r#"
[bans]
deny = [
    { name = "openssl", wrappers = [], reason = "Use rustls" },
    { name = "regex", wrappers = ["tree-sitter", "globset"], reason = "Use structured parsers" },
]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.deny.len(), 2, "should have 2 deny entries");

        let first = match &bans.deny[0] {
            BanDenyEntry::Detailed(d) => d,
            BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
        };
        assert_eq!(first.name(), Some("openssl"), "first entry name");
        assert_eq!(first.reason(), Some("Use rustls"), "first entry reason");
        assert!(first.wrappers().is_empty(), "openssl should have no wrappers");

        let second = match &bans.deny[1] {
            BanDenyEntry::Detailed(d) => d,
            BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
        };
        assert_eq!(second.wrappers().len(), 2, "regex should have 2 wrappers");
        assert_eq!(second.wrappers()[0], "tree-sitter", "first wrapper");
        assert_eq!(second.wrappers()[1], "globset", "second wrapper");
    }

    #[test]
    fn bans_skip_entries() {
        let cfg = parse(
            r#"
[bans]
skip = [
    "windows-sys",
    { name = "syn", version = "=1", reason = "Transitive via proc-macro2" },
]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.skip.len(), 2, "should have 2 skip entries");
        assert_eq!(bans.skip[0].name(), Some("windows-sys"), "first skip name");
        assert_eq!(bans.skip[0].reason(), None, "simple skip has no reason");
        assert_eq!(bans.skip[1].name(), Some("syn"), "second skip name");
        assert_eq!(
            bans.skip[1].reason(),
            Some("Transitive via proc-macro2"),
            "detailed skip reason",
        );
    }

    #[test]
    fn bans_allow_entries() {
        let cfg = parse(
            r#"
[bans]
allow = [
    "serde",
    { name = "tokio", version = "1" },
]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.allow.len(), 2, "should have 2 allow entries");
        assert_eq!(bans.allow[0].name(), Some("serde"), "first allow name");
        assert_eq!(bans.allow[1].name(), Some("tokio"), "second allow name");
    }

    #[test]
    fn bans_feature_entries() {
        let cfg = parse(
            r#"
[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync"]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.features.len(), 1, "should have 1 feature entry");
        let feat = &bans.features[0];
        assert_eq!(feat.name(), Some("tokio"), "feature entry name");
        assert_eq!(feat.deny(), &["full"], "denied features");
        assert_eq!(feat.allow().len(), 4, "should have 4 allowed features");
    }

    #[test]
    fn bans_wildcard_settings() {
        let cfg = parse(
            r#"
[bans]
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.wildcards, Some("allow".into()), "wildcards mismatch");
        assert_eq!(bans.allow_wildcard_paths, Some(true), "allow_wildcard_paths mismatch");
        assert_eq!(bans.highlight, Some("all".into()), "highlight mismatch");
    }

    #[test]
    fn licenses_section_parses() {
        let cfg = parse(
            r#"
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
confidence-threshold = 0.8

[licenses.private]
ignore = true
"#,
        );

        let lic = cfg.licenses.expect("licenses should be present");
        assert_eq!(lic.allow.len(), 3, "should have 3 allowed licenses");
        assert_eq!(lic.allow[0], "MIT", "first license");
        assert_eq!(lic.confidence_threshold, Some(0.8), "confidence threshold mismatch");

        let private = lic.private.expect("private should be present");
        assert_eq!(private.ignore(), Some(true), "private.ignore mismatch");
        assert!(private.registries().is_empty(), "private.registries should be empty");
    }

    #[test]
    fn license_exceptions_parse() {
        let cfg = parse(
            r#"
[licenses]
allow = ["MIT"]
exceptions = [
    { name = "ring", allow = ["OpenSSL"] },
    { name = "unicode-ident", allow = ["Unicode-DFS-2016"] },
]
"#,
        );

        let lic = cfg.licenses.expect("licenses should be present");
        assert_eq!(lic.exceptions.len(), 2, "should have 2 exceptions");
        assert_eq!(lic.exceptions[0].name(), "ring", "first exception name");
        assert_eq!(lic.exceptions[0].allow(), &["OpenSSL"], "first exception allow");
        assert_eq!(lic.exceptions[1].name(), "unicode-ident", "second exception name");
    }

    #[test]
    fn sources_section_parses() {
        let cfg = parse(
            r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
        );

        let src = cfg.sources.expect("sources should be present");
        assert_eq!(src.unknown_registry, Some("deny".into()), "unknown_registry mismatch");
        assert_eq!(src.unknown_git, Some("deny".into()), "unknown_git mismatch");
        assert_eq!(src.allow_registry.len(), 1, "should have 1 allowed registry");
        assert_eq!(
            src.allow_registry[0], "sparse+https://index.crates.io/",
            "registry URL mismatch",
        );
        assert!(src.allow_git.is_empty(), "allow_git should be empty");
        assert!(src.extra.is_empty(), "sources extra should be empty");
    }

    #[test]
    fn output_section_parses() {
        let cfg = parse(
            r#"
[output]
feature-depth = 1
"#,
        );

        let out = cfg.output.expect("output should be present");
        assert_eq!(out.feature_depth, Some(1), "feature_depth mismatch");
        assert!(out.extra.is_empty(), "output extra should be empty");
    }

    #[test]
    fn unknown_top_level_keys_land_in_extra() {
        let cfg = parse(
            r#"
[graph]
all-features = true

[some-future-section]
key = "value"
"#,
        );

        assert!(cfg.graph.is_some(), "graph should parse");
        assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown top-level key");
        assert!(
            cfg.extra.contains_key("some-future-section"),
            "unknown section should be captured",
        );
    }

    #[test]
    fn unknown_keys_in_nested_sections() {
        let cfg = parse(
            r#"
[graph]
all-features = true
some-new-graph-option = "test"

[advisories]
unmaintained = "deny"
new-advisory-field = 42

[bans]
multiple-versions = "deny"
new-ban-option = true

[licenses]
allow = ["MIT"]
new-license-field = "test"

[sources]
unknown-registry = "deny"
new-source-field = false

[output]
feature-depth = 1
new-output-field = "hello"
"#,
        );

        let graph = cfg.graph.expect("graph should be present");
        assert_eq!(graph.extra.len(), 1, "graph should have 1 extra key");

        let adv = cfg.advisories.expect("advisories should be present");
        assert_eq!(adv.extra.len(), 1, "advisories should have 1 extra key");

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.extra.len(), 1, "bans should have 1 extra key");

        let lic = cfg.licenses.expect("licenses should be present");
        assert_eq!(lic.extra.len(), 1, "licenses should have 1 extra key");

        let src = cfg.sources.expect("sources should be present");
        assert_eq!(src.extra.len(), 1, "sources should have 1 extra key");

        let out = cfg.output.expect("output should be present");
        assert_eq!(out.extra.len(), 1, "output should have 1 extra key");
    }

    #[test]
    fn real_deny_toml_parses() {
        let cfg = parse(
            r#"
[graph]
all-features = true
no-default-features = false

[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"

skip = []

deny = [
    { name = "simd-json", wrappers = [], reason = "Ban competing JSON libraries" },
    { name = "openssl", wrappers = [], reason = "Ban OpenSSL (standardize on rustls)" },
    { name = "regex", wrappers = ["tree-sitter", "globset", "ignore"], reason = "Ban regex crates" },
]

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Unicode-3.0",
    "Zlib",
    "CC0-1.0",
    "OpenSSL",
    "BSL-1.0",
    "MPL-2.0",
]
confidence-threshold = 0.8

[licenses.private]
ignore = true

[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
        );

        assert!(cfg.graph.is_some(), "graph should parse");
        assert!(cfg.bans.is_some(), "bans should parse");
        assert!(cfg.licenses.is_some(), "licenses should parse");
        assert!(cfg.advisories.is_some(), "advisories should parse");
        assert!(cfg.sources.is_some(), "sources should parse");
        assert!(cfg.extra.is_empty(), "all top-level keys should be known");

        let bans = cfg.bans.expect("bans present");
        assert_eq!(bans.deny.len(), 3, "should have 3 deny entries");
        assert_eq!(bans.features.len(), 1, "should have 1 feature entry");

        let lic = cfg.licenses.expect("licenses present");
        assert_eq!(lic.allow.len(), 12, "should have 12 allowed licenses");
        assert!(lic.private.is_some(), "private config should be present");
    }

    #[test]
    fn from_str_error_on_invalid_toml() {
        let bad = "this is not [[[valid toml";
        let err = bad.parse::<DenyConfig>();
        assert!(err.is_err(), "invalid TOML should produce an error");

        let msg = err.expect_err("should be an error").to_string();
        assert!(
            msg.contains("invalid deny.toml"),
            "expected error message prefix, got: {msg}",
        );
    }

    #[test]
    fn ban_deny_crate_field_parses() {
        let cfg = parse(
            r#"
[bans]
deny = [
    { crate = "openssl", reason = "Use rustls" },
]
"#,
        );

        let bans = cfg.bans.expect("bans should be present");
        assert_eq!(bans.deny.len(), 1, "should have 1 deny entry");

        let entry = match &bans.deny[0] {
            BanDenyEntry::Detailed(d) => d,
            BanDenyEntry::Simple(_) => panic!("expected detailed entry"),
        };
        assert_eq!(entry.crate_name(), Some("openssl"), "crate field mismatch");
        assert_eq!(entry.name(), None, "name should be None when using crate field");
    }

    #[test]
    fn license_private_with_registries() {
        let cfg = parse(
            r#"
[licenses]
allow = ["MIT"]

[licenses.private]
ignore = true
registries = ["https://my-registry.example.com"]
"#,
        );

        let lic = cfg.licenses.expect("licenses should be present");
        let private = lic.private.expect("private should be present");
        assert_eq!(private.ignore(), Some(true), "private.ignore mismatch");
        assert_eq!(private.registries().len(), 1, "should have 1 registry");
        assert_eq!(
            private.registries()[0], "https://my-registry.example.com",
            "registry URL mismatch",
        );
    }
}
