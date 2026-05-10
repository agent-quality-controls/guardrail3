#![allow(
    clippy::module_name_repetitions,
    reason = "deny.toml schema mirror: types in this module intentionally repeat the deny.toml table name they model"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdvisoryScope {
    All,
    Workspace,
    Transitive,
    None,
}

/// Security advisory checking settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdvisoriesConfig {
    /// Advisory database path.
    pub db_path: Option<String>,
    /// Advisory database URLs.
    #[serde(default)]
    pub db_urls: Vec<String>,
    /// Action for unmaintained crates: `"deny"`, `"warn"`, `"allow"`, or `"workspace"`.
    pub unmaintained: Option<AdvisoryScope>,
    /// Action scope for unsound advisories.
    pub unsound: Option<AdvisoryScope>,
    /// Action for yanked crates: `"deny"`, `"warn"`, `"allow"`.
    pub yanked: Option<String>,
    /// Whether to fetch advisory DBs through the git CLI.
    pub git_fetch_with_cli: Option<bool>,
    /// Whether yanked crate checking is disabled.
    pub disable_yank_checking: Option<bool>,
    /// Maximum allowed advisory database staleness in RFC3339 duration format.
    pub maximum_db_staleness: Option<String>,
    /// Unused ignored advisory handling.
    pub unused_ignored_advisory: Option<String>,
    /// Deprecated version field preserved as parsed data.
    pub version: Option<u32>,
    /// Deprecated: action for vulnerability advisories.
    pub vulnerability: Option<String>,
    /// Deprecated: action for notice advisories.
    pub notice: Option<String>,
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

/// Detailed advisory ignore entry with an ID and optional reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdvisoryIgnoreDetail {
    /// The advisory ID (e.g. `"RUSTSEC-2024-0001"`).
    pub id: Option<String>,
    /// Crate package spec to ignore.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated crate name field.
    pub name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// Why this advisory is being ignored.
    pub reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
