use serde::{Deserialize, Serialize};

/// Parsed representation of a `.cargo/mutants.toml` configuration file.
///
/// All known cargo-mutants configuration keys are mapped to typed fields.
/// Unknown keys are rejected because upstream cargo-mutants uses
/// `deny_unknown_fields` for this file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct MutantsToml {
    #[serde(default)]
    pub exclude_re: Vec<String>,
    #[serde(default)]
    pub examine_re: Vec<String>,
    #[serde(default)]
    pub exclude_globs: Vec<String>,
    #[serde(default)]
    pub examine_globs: Vec<String>,
    #[serde(default)]
    pub skip_calls: Vec<String>,
    pub skip_calls_defaults: Option<bool>,
    #[serde(default)]
    pub error_values: Vec<String>,
    pub timeout_multiplier: Option<f64>,
    pub minimum_test_timeout: Option<f64>,
    pub build_timeout_multiplier: Option<f64>,
    pub all_features: Option<bool>,
    pub no_default_features: Option<bool>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub test_package: Vec<String>,
    pub test_workspace: Option<bool>,
    #[serde(default)]
    pub additional_cargo_args: Vec<String>,
    #[serde(default)]
    pub additional_cargo_test_args: Vec<String>,
    pub profile: Option<String>,
    pub cap_lints: Option<bool>,
    pub copy_vcs: Option<bool>,
    pub copy_target: Option<bool>,
    pub gitignore: Option<bool>,
    pub output: Option<String>,
    pub test_tool: Option<TestTool>,
    pub sharding: Option<Sharding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestTool {
    Cargo,
    Nextest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Sharding {
    RoundRobin,
    Slice,
}
