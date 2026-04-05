use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GitSpec {
    Any,
    Branch,
    Tag,
    Rev,
}

/// Source restriction settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SourcesConfig {
    /// Action for unknown registries: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_registry: Option<String>,
    /// Action for unknown git sources: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_git: Option<String>,
    /// Required git spec level for git dependencies.
    pub required_git_spec: Option<GitSpec>,
    /// Allowed registries.
    #[serde(default)]
    pub allow_registry: Vec<String>,
    /// Allowed git sources.
    #[serde(default)]
    pub allow_git: Vec<String>,
    /// Private git source prefixes.
    #[serde(default)]
    pub private: Vec<String>,
    /// Unused allowed-source handling.
    pub unused_allowed_source: Option<String>,
    /// Trusted organizations or users by forge.
    pub allow_org: Option<SourcesAllowOrg>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Allowed source organizations grouped by forge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SourcesAllowOrg {
    /// Allowed GitHub orgs/users.
    #[serde(default)]
    pub github: Vec<String>,
    /// Allowed GitLab orgs/users.
    #[serde(default)]
    pub gitlab: Vec<String>,
    /// Allowed Bitbucket orgs/users.
    #[serde(default)]
    pub bitbucket: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

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
