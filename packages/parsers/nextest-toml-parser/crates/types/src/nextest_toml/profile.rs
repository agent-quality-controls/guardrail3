#![allow(
    clippy::module_name_repetitions,
    reason = "nextest.toml schema mirror: profile module exposes Profile* types that intentionally repeat the nextest.toml table name"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use super::execution::{
    ArchiveConfig, FailFastConfig, FinalStatusLevel, FlakyResult, NextestBenchConfig, RetryPolicy,
    StatusLevel, TestOutputDisplay, TestThreads, ThreadsRequired, TimeoutConfig,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestProfile {
    pub inherits: Option<String>,
    pub default_filter: Option<String>,
    pub slow_timeout: Option<TimeoutConfig>,
    pub leak_timeout: Option<TimeoutConfig>,
    pub global_timeout: Option<String>,
    pub test_threads: Option<TestThreads>,
    pub threads_required: Option<ThreadsRequired>,
    #[serde(default)]
    pub run_extra_args: Vec<String>,
    pub retries: Option<RetryPolicy>,
    pub flaky_result: Option<FlakyResult>,
    pub status_level: Option<StatusLevel>,
    pub final_status_level: Option<FinalStatusLevel>,
    pub failure_output: Option<TestOutputDisplay>,
    pub success_output: Option<TestOutputDisplay>,
    pub fail_fast: Option<FailFastConfig>,
    pub test_group: Option<String>,
    #[serde(default)]
    pub overrides: Vec<ProfileOverride>,
    #[serde(default)]
    pub scripts: Vec<ProfileScriptConfig>,
    pub junit: Option<JunitConfig>,
    pub archive: Option<ArchiveConfig>,
    pub bench: Option<NextestBenchConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileScriptConfig {
    pub platform: Option<PlatformConfig>,
    pub filter: Option<String>,
    pub setup: Option<ScriptReference>,
    pub list_wrapper: Option<String>,
    pub run_wrapper: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptReference {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileOverride {
    pub filter: Option<String>,
    pub platform: Option<PlatformConfig>,
    pub default_filter: Option<String>,
    pub priority: Option<i32>,
    pub threads_required: Option<ThreadsRequired>,
    #[serde(default)]
    pub run_extra_args: Vec<String>,
    pub retries: Option<RetryPolicy>,
    pub flaky_result: Option<FlakyResult>,
    pub slow_timeout: Option<TimeoutConfig>,
    pub bench: Option<OverrideBenchConfig>,
    pub leak_timeout: Option<TimeoutConfig>,
    pub test_group: Option<String>,
    pub success_output: Option<TestOutputDisplay>,
    pub failure_output: Option<TestOutputDisplay>,
    pub junit: Option<OverrideJunitConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OverrideBenchConfig {
    pub slow_timeout: Option<TimeoutConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OverrideJunitConfig {
    pub store_success_output: Option<bool>,
    pub store_failure_output: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlatformConfig {
    Name(String),
    Detailed(PlatformDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlatformDetail {
    pub host: Option<String>,
    pub target: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JunitConfig {
    pub path: Option<String>,
    pub report_name: Option<String>,
    pub store_success_output: Option<bool>,
    pub store_failure_output: Option<bool>,
    pub flaky_fail_status: Option<JunitFlakyFailStatus>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JunitFlakyFailStatus {
    Failure,
    Success,
}
