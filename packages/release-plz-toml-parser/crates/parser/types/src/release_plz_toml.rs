use std::collections::BTreeMap;

use serde::Deserialize;
use toml::Value;

/// Parsed representation of a `release-plz.toml` configuration file.
///
/// Known release-plz configuration keys are mapped to typed fields. Unknown keys
/// are captured in [`extra`](Self::extra) via `#[serde(flatten)]` for forward
/// compatibility.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReleasePlzToml {
    /// Workspace-level release-plz settings.
    pub workspace: Option<ReleasePlzWorkspace>,
    /// Per-package release-plz overrides.
    #[serde(default)]
    pub package: Vec<ReleasePlzPackage>,
    /// Unknown top-level keys preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Workspace-level release-plz configuration.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReleasePlzWorkspace {
    /// Path to a git-cliff configuration file for changelog generation.
    pub changelog_config: Option<String>,
    /// Whether to create GitHub/GitLab releases.
    pub git_release_enable: Option<bool>,
    /// Whether to always create a release, even when there are no changes.
    pub release_always: Option<bool>,
    /// Unknown workspace keys preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Per-package release-plz configuration override.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReleasePlzPackage {
    /// The package name this override applies to.
    pub name: Option<String>,
    /// Whether this package should be published.
    pub publish: Option<bool>,
    /// Unknown package keys preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
