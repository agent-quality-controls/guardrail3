use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::Error;

/// Parsed representation of a `.cargo/mutants.toml` configuration file.
///
/// All known cargo-mutants configuration keys are mapped to typed fields.
/// Unknown keys are captured in [`extra`](Self::extra) for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct MutantsConfig {
    // === Mutation boundaries ===

    /// Regex patterns for source files to exclude from mutation.
    #[serde(default)]
    pub exclude_re: Vec<String>,

    /// Regex patterns for source files to examine (include).
    #[serde(default)]
    pub examine_re: Vec<String>,

    /// Glob patterns for source files to exclude.
    #[serde(default)]
    pub exclude_globs: Vec<String>,

    /// Glob patterns for source files to examine.
    #[serde(default)]
    pub examine_globs: Vec<String>,

    // === Skip configuration ===

    /// Function call patterns to skip during mutation.
    #[serde(default)]
    pub skip_calls: Vec<String>,

    /// Whether to include default skip patterns.
    pub skip_calls_defaults: Option<bool>,

    // === Custom return values ===

    /// Custom error values for mutation.
    #[serde(default)]
    pub error_values: Vec<String>,

    // === Timeouts ===

    /// Multiplier for test timeout (e.g., 3.0 = 3x normal).
    pub timeout_multiplier: Option<f64>,

    /// Minimum test timeout duration string (e.g., "20s").
    pub minimum_test_timeout: Option<String>,

    /// Multiplier for build timeout.
    pub build_timeout_multiplier: Option<f64>,

    // === Feature control ===

    /// Enable all features when building.
    pub all_features: Option<bool>,

    /// Disable default features when building.
    pub no_default_features: Option<bool>,

    /// Specific features to enable.
    #[serde(default)]
    pub features: Vec<String>,

    // === Test execution ===

    /// Test tool to use ("cargo" or "nextest").
    pub test_tool: Option<String>,

    /// Specific package to test.
    pub test_package: Option<String>,

    /// Whether to test the entire workspace.
    pub test_workspace: Option<bool>,

    // === Cargo integration ===

    /// Additional arguments passed to cargo.
    #[serde(default)]
    pub additional_cargo_args: Vec<String>,

    /// Additional arguments passed to cargo test.
    #[serde(default)]
    pub additional_cargo_test_args: Vec<String>,

    /// Build profile to use.
    pub profile: Option<String>,

    /// Cap lints level.
    pub cap_lints: Option<bool>,

    /// Whether to copy VCS files into the scratch directory.
    pub copy_vcs: Option<bool>,

    // === Output ===

    /// Output directory for results.
    pub output: Option<String>,

    // === Catch-all ===

    /// Unknown configuration keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl MutantsConfig {
    /// Read and parse a mutants.toml file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] on read failure, [`Error::Toml`] on parse failure.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let content = crate::fs::read_to_string(path)?;
        content.parse()
    }
}

impl std::str::FromStr for MutantsConfig {
    type Err = Error;

    #[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized mutants.toml parser — toml::from_str is its core purpose
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}
