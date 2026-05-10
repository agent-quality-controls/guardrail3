#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Maps each `ESLint` plugin alias to the `npm` package names it resolves to during effective-config resolution.
pub type PluginPackageNamesMap = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslintConfigDocument {
    pub raw: Value,
    pub typed: EslintConfigParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EslintConfigParseState {
    Parsed(EslintConfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EslintConfigSnapshot {
    pub selected_config: EslintSelectedConfigFile,
    pub probes: Vec<EslintEffectiveConfigProbe>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EslintSelectedConfigFile {
    pub rel_path: String,
    pub kind: EslintConfigFileKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EslintConfigFileKind {
    Js,
    Mjs,
    Cjs,
    Ts,
    Mts,
    Cts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EslintProbeTarget {
    pub probe: EslintProbeKind,
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EslintEffectiveConfigProbe {
    pub probe: EslintProbeKind,
    pub rel_path: String,
    pub ignored: bool,
    pub plugins: Vec<String>,
    pub plugin_meta_names: BTreeMap<String, String>,
    pub plugin_package_names: PluginPackageNamesMap,
    pub rules: BTreeMap<String, EslintRuleSetting>,
    pub project_service: Option<bool>,
    pub linter_options_no_inline_config: Option<bool>,
    pub linter_options_report_unused_disable_directives: Option<EslintReportUnusedSetting>,
    pub linter_options_report_unused_inline_configs: Option<EslintReportUnusedSetting>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EslintProbeKind {
    AstroSource,
    TsSource,
    TsxSource,
    MdxContent,
    AstroContentConfig,
    TsTest,
    JsSource,
    ConfigFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EslintRuleSetting {
    pub severity: EslintRuleSeverity,
    pub options: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EslintRuleSeverity {
    Off,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EslintReportUnusedSetting {
    Off,
    Warn,
    Error,
}
