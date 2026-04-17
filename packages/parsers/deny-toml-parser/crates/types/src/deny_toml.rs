use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::advisories::AdvisoriesConfig;
use crate::bans::BansConfig;
use crate::graph::GraphConfig;
use crate::licenses::LicensesConfig;
use crate::sources::{OutputConfig, SourcesConfig};

/// Parsed representation of a `deny.toml` configuration file.
///
/// All known top-level sections are mapped to typed fields. Unknown keys are
/// captured in [`extra`](Self::extra) for forward compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct DenyToml {
    /// Dependency graph configuration.
    pub graph: Option<GraphConfig>,
    /// Security advisory checking settings.
    pub advisories: Option<AdvisoriesConfig>,
    /// Dependency ban settings.
    pub bans: Option<BansConfig>,
    /// License checking settings.
    pub licenses: Option<LicensesConfig>,
    /// Standalone exceptions file shape used by `deny.exceptions.toml`.
    #[serde(default)]
    pub exceptions: Vec<crate::licenses::LicenseException>,
    /// Source restrictions.
    pub sources: Option<SourcesConfig>,
    /// Output formatting configuration.
    pub output: Option<OutputConfig>,
    /// Unknown top-level keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
