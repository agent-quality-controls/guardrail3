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
    pub targets: Vec<GraphTargetEntry>,
    /// Crates to exclude from all checks.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Features to enable when collecting metadata.
    #[serde(default)]
    pub features: Vec<String>,
    /// Whether dev-dependencies should be excluded from the graph.
    pub exclude_dev: Option<bool>,
    /// Whether unpublished workspace crates should be excluded as graph roots.
    pub exclude_unpublished: Option<bool>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A graph target entry, either a bare triple string or a detailed table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraphTargetEntry {
    /// Bare target triple string.
    Simple(String),
    /// Detailed target configuration.
    Detailed(GraphTargetDetail),
}

/// Detailed graph target configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GraphTargetDetail {
    /// Target triple to filter for.
    pub triple: String,
    /// Target features enabled for that triple.
    #[serde(default)]
    pub features: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
