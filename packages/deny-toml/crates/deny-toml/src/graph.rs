use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

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
