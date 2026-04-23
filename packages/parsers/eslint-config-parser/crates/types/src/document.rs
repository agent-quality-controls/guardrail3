use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct EslintConfigDocument {
    pub raw: Value,
    pub typed: EslintConfigParseState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EslintConfigParseState {
    Parsed(EslintConfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EslintEffectiveConfigProbe {
    pub probe: EslintProbeKind,
    pub rel_path: String,
    pub ignored: bool,
    pub plugins: Vec<String>,
    pub rules: BTreeMap<String, EslintRuleSetting>,
    pub project_service: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EslintProbeKind {
    AstroSource,
    TsSource,
    TsxSource,
    TsTest,
    JsSource,
    ConfigFile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
