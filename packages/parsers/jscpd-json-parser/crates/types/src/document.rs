use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct JscpdDocument {
    pub raw: Value,
    pub typed: JscpdParseState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JscpdParseState {
    Parsed(JscpdSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JscpdSnapshot {
    #[serde(default)]
    pub threshold: Option<i64>,
    #[serde(rename = "minTokens", default)]
    pub min_tokens: Option<u64>,
    #[serde(default)]
    pub absolute: Option<bool>,
    #[serde(default)]
    pub format: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default = "default_extra_keys")]
    pub extra_keys: Vec<String>,
}

fn default_extra_keys() -> Vec<String> {
    Vec::new()
}
