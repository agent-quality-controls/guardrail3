use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

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
