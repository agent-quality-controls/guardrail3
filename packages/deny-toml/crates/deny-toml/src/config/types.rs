use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::advisories::AdvisoriesConfig;
use crate::bans::BansConfig;
use crate::graph::GraphConfig;
use crate::licenses::LicensesConfig;
use crate::sources::{OutputConfig, SourcesConfig};
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
