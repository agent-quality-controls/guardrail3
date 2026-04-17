use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use super::execution::{RelativeTo, TimeoutConfig};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScriptsConfig {
    #[serde(default)]
    pub setup: BTreeMap<String, SetupScriptConfig>,
    #[serde(default)]
    pub wrapper: BTreeMap<String, WrapperScriptConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetupScriptConfig {
    pub command: ScriptCommand,
    pub slow_timeout: Option<TimeoutConfig>,
    pub leak_timeout: Option<TimeoutConfig>,
    pub capture_stdout: Option<bool>,
    pub capture_stderr: Option<bool>,
    pub junit: Option<ScriptJunitConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WrapperScriptConfig {
    pub command: ScriptCommand,
    pub target_runner: Option<TargetRunnerMode>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptCommand {
    Simple(String),
    Argv(Vec<String>),
    Detailed(ScriptCommandDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScriptCommandDetail {
    pub command_line: String,
    pub relative_to: Option<RelativeTo>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetRunnerMode {
    Ignore,
    OverridesWrapper,
    WithinWrapper,
    AroundWrapper,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScriptJunitConfig {
    pub store_success_output: Option<bool>,
    pub store_failure_output: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
