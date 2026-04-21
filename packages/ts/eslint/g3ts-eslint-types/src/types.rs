use std::collections::BTreeMap;

use eslint_config_parser::types::{EslintConfigFileKind, EslintProbeKind, EslintRuleSeverity};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsEslintSelectedConfig {
    pub rel_path: String,
    pub kind: EslintConfigFileKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsEslintRuleSetting {
    pub severity: EslintRuleSeverity,
    pub options: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsEslintEffectiveConfigProbe {
    pub probe: EslintProbeKind,
    pub rel_path: String,
    pub ignored: bool,
    pub plugins: Vec<String>,
    pub rules: BTreeMap<String, G3TsEslintRuleSetting>,
    pub project_service: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsEslintConfigSnapshot {
    pub selected_config: G3TsEslintSelectedConfig,
    pub probes: Vec<G3TsEslintEffectiveConfigProbe>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsEslintConfigState {
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsEslintConfigSnapshot },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsEslintConfigChecksInput {
    pub config: G3TsEslintConfigState,
}
