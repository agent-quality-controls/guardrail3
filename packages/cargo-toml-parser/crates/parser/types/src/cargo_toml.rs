#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors Cargo.toml schema directly; field names intentionally track TOML keys"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Typed representation of a `Cargo.toml` file.
///
/// The model intentionally focuses on file-local manifest structure rather than
/// Cargo workspace inheritance resolution. Known sections are typed, while
/// unknown keys are preserved in `extra`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CargoToml {
    pub package: Option<PackageSection>,
    pub workspace: Option<WorkspaceSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "dev-dependencies", default)]
    pub dev_dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "build-dependencies", default)]
    pub build_dependencies: BTreeMap<String, Dependency>,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
    pub lints: Option<LintsConfig>,
    #[serde(default)]
    pub target: BTreeMap<String, TargetDependencyTables>,
    pub lib: Option<NamedTarget>,
    #[serde(default)]
    pub bin: Vec<NamedTarget>,
    #[serde(default)]
    pub example: Vec<NamedTarget>,
    #[serde(default)]
    pub test: Vec<NamedTarget>,
    #[serde(default)]
    pub bench: Vec<NamedTarget>,
    #[serde(default)]
    pub profile: BTreeMap<String, Value>,
    #[serde(default)]
    pub patch: BTreeMap<String, Value>,
    #[serde(default)]
    pub replace: BTreeMap<String, Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct PackageSection {
    pub name: Option<String>,
    pub version: Option<String>,
    pub edition: Option<String>,
    pub rust_version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    pub publish: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct WorkspaceSection {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(rename = "default-members", default)]
    pub default_members: Vec<String>,
    pub resolver: Option<String>,
    pub package: Option<WorkspacePackageSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    pub lints: Option<LintsConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct WorkspacePackageSection {
    pub version: Option<String>,
    pub edition: Option<String>,
    pub rust_version: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DependencyDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct DependencyDetail {
    pub version: Option<String>,
    pub path: Option<String>,
    pub registry: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub package: Option<String>,
    pub workspace: Option<bool>,
    pub optional: Option<bool>,
    #[serde(rename = "default-features")]
    pub default_features: Option<bool>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintValue {
    Level(String),
    Detailed(LintDetail),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct LintDetail {
    pub level: String,
    pub priority: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct LintsConfig {
    pub workspace: Option<bool>,
    #[serde(default)]
    pub rust: BTreeMap<String, LintValue>,
    #[serde(default)]
    pub clippy: BTreeMap<String, LintValue>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TargetDependencyTables {
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "dev-dependencies", default)]
    pub dev_dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "build-dependencies", default)]
    pub build_dependencies: BTreeMap<String, Dependency>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NamedTarget {
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(default)]
    pub required_features: Vec<String>,
    pub test: Option<bool>,
    pub doctest: Option<bool>,
    pub bench: Option<bool>,
    pub doc: Option<bool>,
    pub harness: Option<bool>,
    #[serde(rename = "proc-macro")]
    pub proc_macro: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
