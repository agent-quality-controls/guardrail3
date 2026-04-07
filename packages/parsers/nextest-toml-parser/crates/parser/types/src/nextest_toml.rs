#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors nextest.toml schema directly; field names intentionally track the file shape"
)]

use std::collections::BTreeMap;

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use toml::Value;

/// Parsed representation of a `.config/nextest.toml` configuration file.
///
/// Known nextest keys are typed where the contract is explicit in nextest's
/// configuration reference. Unknown keys are preserved in `extra` because
/// nextest itself warns on unknown configuration and otherwise ignores it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestToml {
    pub store: Option<StoreConfig>,
    pub nextest_version: Option<NextestVersionConfig>,
    #[serde(default)]
    pub experimental: Vec<ExperimentalFeature>,
    #[serde(default)]
    pub test_groups: BTreeMap<String, TestGroupConfig>,
    #[serde(default, rename = "script")]
    pub script: BTreeMap<String, SetupScriptConfig>,
    pub scripts: Option<ScriptsConfig>,
    #[serde(default)]
    pub profile: BTreeMap<String, NextestProfile>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for NextestToml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawNextestToml {
            store: Option<StoreConfig>,
            nextest_version: Option<NextestVersionConfig>,
            #[serde(default)]
            experimental: Vec<ExperimentalFeature>,
            #[serde(default)]
            test_groups: BTreeMap<String, TestGroupConfig>,
            #[serde(default, rename = "script")]
            script: BTreeMap<String, SetupScriptConfig>,
            scripts: Option<ScriptsConfig>,
            #[serde(default)]
            profile: BTreeMap<String, NextestProfile>,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let raw = RawNextestToml::deserialize(deserializer)?;
        let scripts = raw.scripts.as_ref();
        let has_legacy_setup = !raw.script.is_empty();
        let has_setup_scripts = scripts.is_some_and(|scripts| !scripts.setup.is_empty());
        let has_wrapper_scripts = scripts.is_some_and(|scripts| !scripts.wrapper.is_empty());
        let has_experimental_setup = raw.experimental.contains(&ExperimentalFeature::SetupScripts);
        let has_experimental_wrapper =
            raw.experimental.contains(&ExperimentalFeature::WrapperScripts);

        if has_legacy_setup && has_setup_scripts {
            return Err(de::Error::custom(
                "invalid nextest.toml: [script.*] cannot be used together with [scripts.setup.*]",
            ));
        }

        if (has_legacy_setup || has_setup_scripts) && !has_experimental_setup {
            return Err(de::Error::custom(
                "invalid nextest.toml: setup scripts require experimental = [\"setup-scripts\"]",
            ));
        }

        if has_wrapper_scripts && !has_experimental_wrapper {
            return Err(de::Error::custom(
                "invalid nextest.toml: wrapper scripts require experimental = [\"wrapper-scripts\"]",
            ));
        }

        Ok(Self {
            store: raw.store,
            nextest_version: raw.nextest_version,
            experimental: raw.experimental,
            test_groups: raw.test_groups,
            script: raw.script,
            scripts: raw.scripts,
            profile: raw.profile,
            extra: raw.extra,
        })
    }
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScriptJunitConfig {
    pub store_success_output: Option<bool>,
    pub store_failure_output: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ArchiveConfig {
    #[serde(default)]
    pub include: Vec<ArchiveInclude>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ArchiveInclude {
    pub path: String,
    pub relative_to: Option<RelativeTo>,
    pub depth: Option<ArchiveDepth>,
    pub on_missing: Option<ArchiveOnMissing>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelativeTo {
    Target,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ArchiveDepth {
    Count(u32),
    Infinite,
}

impl<'de> Deserialize<'de> for ArchiveDepth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl serde::de::Visitor<'_> for V {
            type Value = ArchiveDepth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a non-negative integer or the string \"infinite\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "infinite" {
                    Ok(ArchiveDepth::Infinite)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = u32::try_from(v)
                    .map_err(|_| E::invalid_value(de::Unexpected::Unsigned(v), &self))?;
                Ok(ArchiveDepth::Count(value))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(E::invalid_value(de::Unexpected::Signed(v), &self));
                }
                let value =
                    u32::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Signed(v), &self))?;
                Ok(ArchiveDepth::Count(value))
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArchiveOnMissing {
    Ignore,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestGroupConfig {
    pub max_threads: Option<TestGroupMaxThreads>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TestGroupMaxThreads {
    Count(u32),
    NumCpus,
}

impl<'de> Deserialize<'de> for TestGroupMaxThreads {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl serde::de::Visitor<'_> for V {
            type Value = TestGroupMaxThreads;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a positive integer or the string \"num-cpus\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "num-cpus" {
                    Ok(TestGroupMaxThreads::NumCpus)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == 0 {
                    return Err(E::invalid_value(de::Unexpected::Unsigned(v), &self));
                }
                let value = u32::try_from(v)
                    .map_err(|_| E::invalid_value(de::Unexpected::Unsigned(v), &self))?;
                Ok(TestGroupMaxThreads::Count(value))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v <= 0 {
                    return Err(E::invalid_value(de::Unexpected::Signed(v), &self));
                }
                let value =
                    u32::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Signed(v), &self))?;
                Ok(TestGroupMaxThreads::Count(value))
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TestThreads {
    Count(i64),
    NumCpus,
}

impl<'de> Deserialize<'de> for TestThreads {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl serde::de::Visitor<'_> for V {
            type Value = TestThreads;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or the string \"num-cpus\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "num-cpus" {
                    Ok(TestThreads::NumCpus)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == 0 {
                    Err(E::invalid_value(de::Unexpected::Signed(v), &self))
                } else {
                    Ok(TestThreads::Count(v))
                }
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ThreadsRequired {
    Count(i64),
    NumCpus,
    NumTestThreads,
}

impl<'de> Deserialize<'de> for ThreadsRequired {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl serde::de::Visitor<'_> for V {
            type Value = ThreadsRequired;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "an integer, the string \"num-cpus\" or the string \"num-test-threads\""
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "num-cpus" => Ok(ThreadsRequired::NumCpus),
                    "num-test-threads" => Ok(ThreadsRequired::NumTestThreads),
                    _ => Err(E::invalid_value(de::Unexpected::Str(v), &self)),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v <= 0 {
                    Err(E::invalid_value(de::Unexpected::Signed(v), &self))
                } else {
                    Ok(ThreadsRequired::Count(v))
                }
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlakyResult {
    Fail,
    Pass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatusLevel {
    None,
    Fail,
    Retry,
    Slow,
    Leak,
    Pass,
    Skip,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FinalStatusLevel {
    None,
    Fail,
    Flaky,
    Slow,
    Skip,
    Leak,
    Pass,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestOutputDisplay {
    Immediate,
    ImmediateFinal,
    Final,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TerminateMode {
    Wait,
    Immediate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FailFastConfig {
    Bool(bool),
    Detailed(FailFastDetail),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct FailFastDetail {
    pub max_fail: FailFastCount,
    pub terminate: Option<TerminateMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FailFastCount {
    Count(i64),
    All,
}

impl<'de> Deserialize<'de> for FailFastCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl serde::de::Visitor<'_> for V {
            type Value = FailFastCount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a positive integer or the string \"all\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "all" {
                    Ok(FailFastCount::All)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v > 0 {
                    Ok(FailFastCount::Count(v))
                } else {
                    Err(E::invalid_value(de::Unexpected::Signed(v), &self))
                }
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RetryPolicy {
    Count(u32),
    Fixed(RetryPolicyDetail),
    Exponential(ExponentialRetryPolicyDetail),
}

impl<'de> Deserialize<'de> for RetryPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "backoff", rename_all = "kebab-case", deny_unknown_fields)]
        enum RetryPolicySerde {
            #[serde(rename_all = "kebab-case")]
            Fixed {
                count: u32,
                #[serde(default)]
                delay: Option<String>,
                #[serde(default)]
                jitter: bool,
            },
            #[serde(rename_all = "kebab-case")]
            Exponential {
                count: u32,
                delay: String,
                #[serde(default)]
                jitter: bool,
                #[serde(default)]
                max_delay: Option<String>,
            },
        }

        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = RetryPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a table or a non-negative integer")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(E::invalid_value(de::Unexpected::Signed(v), &self));
                }
                let value =
                    u32::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Signed(v), &self))?;
                Ok(RetryPolicy::Count(value))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                match RetryPolicySerde::deserialize(de::value::MapAccessDeserializer::new(map))? {
                    RetryPolicySerde::Fixed {
                        count,
                        delay,
                        jitter,
                    } => Ok(RetryPolicy::Fixed(RetryPolicyDetail {
                        count,
                        delay,
                        jitter,
                    })),
                    RetryPolicySerde::Exponential {
                        count,
                        delay,
                        jitter,
                        max_delay,
                    } => Ok(RetryPolicy::Exponential(ExponentialRetryPolicyDetail {
                        count,
                        delay,
                        jitter,
                        max_delay,
                    })),
                }
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyDetail {
    pub count: u32,
    pub delay: Option<String>,
    pub jitter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct ExponentialRetryPolicyDetail {
    pub count: u32,
    pub delay: String,
    pub jitter: bool,
    pub max_delay: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NextestBenchConfig {
    pub global_timeout: Option<String>,
    pub slow_timeout: Option<TimeoutConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimeoutConfig {
    Simple(String),
    Detailed(TimeoutDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeoutDetail {
    pub period: String,
    pub terminate_after: Option<u32>,
    pub grace_period: Option<String>,
    pub on_timeout: Option<TimeoutResult>,
    pub result: Option<TimeoutResult>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimeoutResult {
    Fail,
    Pass,
}
