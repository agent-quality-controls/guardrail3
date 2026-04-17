use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StoreConfig {
    pub dir: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NextestVersionConfig {
    Simple(String),
    Detailed(NextestVersionDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NextestVersionDetail {
    pub required: Option<String>,
    pub recommended: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExperimentalFeature {
    SetupScripts,
    WrapperScripts,
}
