#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors Cargo config schema directly; field names intentionally track TOML keys"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
use toml::Value;

/// Typed representation of a `.cargo/config.toml` / `.cargo/config` file.
///
/// Known Cargo config sections are mapped to typed fields. Unknown top-level
/// keys are captured in [`extra`](Self::extra) so the model can stay forward
/// compatible as Cargo evolves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct CargoConfigToml {
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub include: Vec<IncludeEntry>,
    #[serde(default)]
    pub alias: BTreeMap<String, CommandValue>,
    pub build: Option<BuildConfig>,
    #[serde(default)]
    pub credential_alias: BTreeMap<String, CommandValue>,
    pub doc: Option<DocConfig>,
    #[serde(default)]
    pub env: BTreeMap<String, EnvValue>,
    pub future_incompat_report: Option<FutureIncompatReportConfig>,
    pub cache: Option<CacheConfig>,
    pub cargo_new: Option<CargoNewConfig>,
    pub http: Option<HttpConfig>,
    pub install: Option<InstallConfig>,
    pub net: Option<NetConfig>,
    #[serde(default)]
    pub patch: BTreeMap<String, Value>,
    #[serde(default)]
    pub profile: BTreeMap<String, ProfileConfig>,
    pub resolver: Option<ResolverConfig>,
    #[serde(default)]
    pub registries: BTreeMap<String, RegistryConfig>,
    pub registry: Option<RegistryDefaults>,
    #[serde(default)]
    pub source: BTreeMap<String, SourceConfig>,
    #[serde(default)]
    pub target: BTreeMap<String, TargetConfig>,
    pub term: Option<TermConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncludeEntry {
    Path(String),
    Detailed(IncludePath),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct IncludePath {
    pub path: String,
    pub optional: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for IncludeEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawIncludeEntry {
            Path(String),
            Detailed(IncludePath),
        }

        let raw = RawIncludeEntry::deserialize(deserializer)?;

        match raw {
            RawIncludeEntry::Path(path) => {
                validate_include_path(&path).map_err(serde::de::Error::custom)?;
                Ok(Self::Path(path))
            }
            RawIncludeEntry::Detailed(detail) => {
                validate_include_path(&detail.path).map_err(serde::de::Error::custom)?;
                Ok(Self::Detailed(detail))
            }
        }
    }
}

#[allow(
    clippy::case_sensitive_file_extension_comparisons,
    reason = "Cargo itself rejects non-lowercase `.toml` include paths"
)]
fn validate_include_path(path: &str) -> Result<(), String> {
    if path.ends_with(".toml") {
        Ok(())
    } else {
        Err(format!(
            "expected a config include path ending with `.toml`, but found `{path}`",
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandValue {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TargetSelector {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegerOrString {
    Integer(u32),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegerOrBool {
    Integer(u32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct BuildConfig {
    pub jobs: Option<IntegerOrString>,
    pub rustc: Option<String>,
    pub rustc_wrapper: Option<String>,
    pub rustc_workspace_wrapper: Option<String>,
    pub rustdoc: Option<String>,
    pub target: Option<TargetSelector>,
    pub target_dir: Option<String>,
    pub build_dir: Option<String>,
    pub rustflags: Option<CommandValue>,
    pub rustdocflags: Option<CommandValue>,
    pub incremental: Option<bool>,
    pub dep_info_basedir: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct DocConfig {
    pub browser: Option<CommandValue>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvValue {
    Simple(String),
    Detailed(EnvValueDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct EnvValueDetail {
    pub value: String,
    pub force: Option<bool>,
    pub relative: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct FutureIncompatReportConfig {
    pub frequency: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct CacheConfig {
    pub auto_clean_frequency: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct CargoNewConfig {
    pub vcs: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpSslVersion {
    String(String),
    Range(HttpTlsRange),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct HttpTlsRange {
    pub min: Option<String>,
    pub max: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct HttpConfig {
    pub debug: Option<bool>,
    pub proxy: Option<String>,
    pub ssl_version: Option<HttpSslVersion>,
    pub timeout: Option<u64>,
    pub low_speed_limit: Option<u64>,
    pub cainfo: Option<String>,
    pub proxy_cainfo: Option<String>,
    pub check_revoke: Option<bool>,
    pub multiplexing: Option<bool>,
    pub user_agent: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct InstallConfig {
    pub root: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NetConfig {
    pub retry: Option<u64>,
    pub git_fetch_with_cli: Option<bool>,
    pub offline: Option<bool>,
    pub ssh: Option<NetSshConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NetSshConfig {
    #[serde(default)]
    pub known_hosts: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ProfileConfig {
    pub inherits: Option<String>,
    pub opt_level: Option<IntegerOrString>,
    pub debug: Option<IntegerOrBool>,
    pub split_debuginfo: Option<String>,
    pub strip: Option<StringOrBool>,
    pub debug_assertions: Option<bool>,
    pub overflow_checks: Option<bool>,
    pub lto: Option<StringOrBool>,
    pub panic: Option<String>,
    pub incremental: Option<bool>,
    pub codegen_units: Option<u32>,
    pub rpath: Option<bool>,
    pub build_override: Option<ProfileSettings>,
    #[serde(default)]
    pub package: BTreeMap<String, ProfileSettings>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ProfileSettings {
    pub opt_level: Option<IntegerOrString>,
    pub debug: Option<IntegerOrBool>,
    pub split_debuginfo: Option<String>,
    pub strip: Option<StringOrBool>,
    pub debug_assertions: Option<bool>,
    pub overflow_checks: Option<bool>,
    pub incremental: Option<bool>,
    pub codegen_units: Option<u32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ResolverConfig {
    pub incompatible_rust_versions: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct RegistryConfig {
    pub index: Option<String>,
    pub token: Option<String>,
    pub credential_provider: Option<CommandValue>,
    pub protocol: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct RegistryDefaults {
    pub index: Option<String>,
    pub default: Option<String>,
    pub credential_provider: Option<CommandValue>,
    pub token: Option<String>,
    #[serde(default)]
    pub global_credential_providers: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct SourceConfig {
    pub replace_with: Option<String>,
    pub directory: Option<String>,
    pub registry: Option<String>,
    pub local_registry: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TargetConfig {
    pub linker: Option<String>,
    pub runner: Option<CommandValue>,
    pub rustflags: Option<CommandValue>,
    pub rustdocflags: Option<CommandValue>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TermConfig {
    pub quiet: Option<bool>,
    pub verbose: Option<bool>,
    pub color: Option<String>,
    pub hyperlinks: Option<bool>,
    pub unicode: Option<bool>,
    pub progress: Option<TermProgressConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TermProgressConfig {
    pub when: Option<String>,
    pub width: Option<u32>,
    pub term_integration: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
