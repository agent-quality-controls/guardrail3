#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StylelintConfigDocument {
    pub raw: Value,
    pub typed: StylelintConfigParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StylelintConfigParseState {
    Parsed(StylelintConfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StylelintConfigSnapshot {
    pub selected_config: StylelintSelectedConfigFile,
    pub raw_extends: Vec<String>,
    pub raw_plugins: Vec<String>,
    pub probes: Vec<StylelintEffectiveConfigProbe>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StylelintSelectedConfigFile {
    pub rel_path: String,
    pub kind: StylelintConfigFileKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StylelintConfigFileKind {
    Js,
    Mjs,
    Cjs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StylelintProbeTarget {
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StylelintEffectiveConfigProbe {
    pub rel_path: String,
    pub ignored: bool,
    pub extends: Vec<String>,
    pub plugins: Vec<String>,
    pub rules: BTreeMap<String, Value>,
}
