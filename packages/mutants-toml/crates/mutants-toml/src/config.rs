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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> MutantsConfig {
        input.parse::<MutantsConfig>().expect("should parse valid mutants.toml")
    }

    #[test]
    fn empty_string_yields_empty_config() {
        let cfg = parse("");

        assert!(cfg.exclude_re.is_empty(), "exclude_re should be empty");
        assert!(cfg.examine_re.is_empty(), "examine_re should be empty");
        assert_eq!(cfg.timeout_multiplier, None, "timeout_multiplier should be None");
        assert_eq!(cfg.test_tool, None, "test_tool should be None");
        assert!(cfg.extra.is_empty(), "extra should be empty");
    }

    #[test]
    fn realistic_config_parses_typed_fields() {
        let cfg = parse(r#"
timeout_multiplier = 3.0
minimum_test_timeout = "20s"
test_tool = "nextest"
exclude_re = ["^tests/", "^benches/"]
examine_globs = ["src/**/*.rs"]
"#);

        assert_eq!(cfg.timeout_multiplier, Some(3.0), "timeout_multiplier mismatch");
        assert_eq!(cfg.minimum_test_timeout.as_deref(), Some("20s"), "minimum_test_timeout mismatch");
        assert_eq!(cfg.test_tool.as_deref(), Some("nextest"), "test_tool mismatch");
        assert_eq!(cfg.exclude_re, vec!["^tests/", "^benches/"], "exclude_re mismatch");
        assert_eq!(cfg.examine_globs, vec!["src/**/*.rs"], "examine_globs mismatch");
        assert!(cfg.extra.is_empty(), "known keys should not land in extra");
    }

    #[test]
    fn unknown_keys_land_in_extra() {
        let cfg = parse(r#"
timeout_multiplier = 2.0
some_future_option = true
"#);

        assert_eq!(cfg.timeout_multiplier, Some(2.0), "known key should parse");
        assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown key");
        assert_eq!(
            cfg.extra.get("some_future_option").and_then(Value::as_bool),
            Some(true),
            "unknown bool key should be captured",
        );
    }

    #[test]
    fn feature_control_fields() {
        let cfg = parse(r#"
all_features = true
no_default_features = false
features = ["serde", "derive"]
"#);

        assert_eq!(cfg.all_features, Some(true), "all_features mismatch");
        assert_eq!(cfg.no_default_features, Some(false), "no_default_features mismatch");
        assert_eq!(cfg.features, vec!["serde", "derive"], "features mismatch");
    }

    #[test]
    fn from_str_error_on_invalid_toml() {
        let bad = "this is not [[[valid toml";
        let err = bad.parse::<MutantsConfig>();
        assert!(err.is_err(), "invalid TOML should produce an error");

        let msg = err.expect_err("should be an error").to_string();
        assert!(
            msg.contains("invalid mutants.toml"),
            "expected error message prefix, got: {msg}",
        );
    }
}
