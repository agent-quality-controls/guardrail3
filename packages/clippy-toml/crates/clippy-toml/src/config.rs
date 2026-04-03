use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::Error;

/// Parsed representation of a `clippy.toml` / `.clippy.toml` configuration file.
///
/// All known Clippy configuration keys are mapped to typed fields.
/// Unknown keys are captured in [`extra`](Self::extra) for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)] // reason: clippy.toml has 10+ boolean configuration options — struct mirrors the schema
pub struct ClippyConfig {
    // === Thresholds ===

    /// Maximum bool fields in a struct before `clippy::struct_excessive_bools` fires.
    pub max_struct_bools: Option<u32>,

    /// Maximum bool parameters in a function.
    pub max_fn_params_bools: Option<u32>,

    /// Maximum lines per function before `clippy::too_many_lines` fires.
    pub too_many_lines_threshold: Option<u32>,

    /// Maximum function parameters.
    pub too_many_arguments_threshold: Option<u32>,

    /// Maximum control-flow nesting depth.
    pub excessive_nesting_threshold: Option<u32>,

    /// Maximum cognitive complexity score per function.
    pub cognitive_complexity_threshold: Option<u32>,

    /// Maximum type nesting complexity score.
    pub type_complexity_threshold: Option<u32>,

    // === Ban lists ===

    /// Banned method calls.
    #[serde(default)]
    pub disallowed_methods: Vec<BanEntry>,

    /// Banned type usages.
    #[serde(default)]
    pub disallowed_types: Vec<BanEntry>,

    /// Banned macro invocations.
    #[serde(default)]
    pub disallowed_macros: Vec<BanEntry>,

    // === Test relaxations ===

    /// Allow `dbg!` in test code.
    pub allow_dbg_in_tests: Option<bool>,

    /// Allow `print!`/`println!` in test code.
    pub allow_print_in_tests: Option<bool>,

    /// Allow `.expect()` in test code.
    pub allow_expect_in_tests: Option<bool>,

    /// Allow `panic!` in test code.
    pub allow_panic_in_tests: Option<bool>,

    /// Allow `.unwrap()` in test code.
    pub allow_unwrap_in_tests: Option<bool>,

    // === Library settings ===

    /// Suppress lints that would break the public API.
    pub avoid_breaking_exported_api: Option<bool>,

    // === Other known settings ===

    /// Minimum Supported Rust Version.
    pub msrv: Option<String>,

    /// Stack size threshold in bytes for `clippy::large_stack_arrays`.
    pub too_large_for_stack: Option<u64>,

    /// Allowed crates for absolute path lints.
    #[serde(default)]
    pub absolute_paths_allowed_crates: Vec<String>,

    /// Maximum path segments for absolute path lints.
    pub absolute_paths_max_segments: Option<u32>,

    /// Types to ignore for interior mutability lints.
    #[serde(default)]
    pub ignore_interior_mutability: Vec<String>,

    /// Types to flag for `clippy::await_holding_invalid_types`.
    #[serde(default)]
    pub await_holding_invalid_types: Vec<Value>,

    /// Allowed operations for `clippy::arithmetic_side_effects`.
    #[serde(default)]
    pub arithmetic_side_effects_allowed: Vec<Value>,

    /// Whether to lint commented-out code.
    pub lint_commented_code: Option<bool>,

    // === Catch-all ===

    /// Unknown configuration keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A ban entry in `disallowed-methods`, `disallowed-types`, or `disallowed-macros`.
///
/// Supports both plain path strings and `{path, reason}` table format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanEntry {
    /// Simple path string: `"std::env::var"`.
    Simple(String),
    /// Detailed entry with reason: `{ path = "std::env::var", reason = "..." }`.
    Detailed(BanEntryDetail),
}

impl BanEntry {
    /// Returns the banned path regardless of entry format.
    #[must_use]
    pub fn path(&self) -> &str {
        match self {
            Self::Simple(path) => path,
            Self::Detailed(detail) => &detail.path,
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason.as_deref(),
        }
    }
}

/// Detailed ban entry with path and optional reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BanEntryDetail {
    /// The banned path (e.g., `"std::env::var"`).
    pub path: String,
    /// Why this item is banned.
    pub reason: Option<String>,
    /// Catch-all for additional fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl ClippyConfig {
    /// Read and parse a clippy.toml file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] on read failure, [`Error::Toml`] on parse failure.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let content = crate::fs::read_to_string(path)?;
        content.parse()
    }
}

impl std::str::FromStr for ClippyConfig {
    type Err = Error;

    #[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized clippy.toml parser — toml::from_str is its core purpose
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ClippyConfig {
        input.parse::<ClippyConfig>().expect("should parse valid clippy.toml")
    }

    #[test]
    fn empty_string_yields_empty_config() {
        let cfg = parse("");

        assert_eq!(cfg.max_struct_bools, None, "max_struct_bools should be None");
        assert_eq!(cfg.cognitive_complexity_threshold, None, "threshold should be None");
        assert!(cfg.disallowed_methods.is_empty(), "disallowed_methods should be empty");
        assert!(cfg.disallowed_types.is_empty(), "disallowed_types should be empty");
        assert!(cfg.disallowed_macros.is_empty(), "disallowed_macros should be empty");
        assert!(cfg.extra.is_empty(), "extra should be empty");
    }

    #[test]
    fn thresholds_parse() {
        let cfg = parse(r#"
max-struct-bools = 3
max-fn-params-bools = 3
too-many-lines-threshold = 75
too-many-arguments-threshold = 7
excessive-nesting-threshold = 4
cognitive-complexity-threshold = 15
type-complexity-threshold = 75
"#);

        assert_eq!(cfg.max_struct_bools, Some(3), "max_struct_bools mismatch");
        assert_eq!(cfg.max_fn_params_bools, Some(3), "max_fn_params_bools mismatch");
        assert_eq!(cfg.too_many_lines_threshold, Some(75), "too_many_lines mismatch");
        assert_eq!(cfg.too_many_arguments_threshold, Some(7), "too_many_arguments mismatch");
        assert_eq!(cfg.excessive_nesting_threshold, Some(4), "excessive_nesting mismatch");
        assert_eq!(cfg.cognitive_complexity_threshold, Some(15), "cognitive_complexity mismatch");
        assert_eq!(cfg.type_complexity_threshold, Some(75), "type_complexity mismatch");
    }

    #[test]
    fn simple_ban_entries() {
        let cfg = parse(r#"
disallowed-methods = ["std::env::var", "std::process::exit"]
disallowed-types = ["std::collections::HashMap"]
disallowed-macros = ["println!", "dbg!"]
"#);

        assert_eq!(cfg.disallowed_methods.len(), 2, "should have 2 method bans");
        assert_eq!(cfg.disallowed_methods[0].path(), "std::env::var", "first method path");
        assert_eq!(cfg.disallowed_methods[0].reason(), None, "simple entry has no reason");

        assert_eq!(cfg.disallowed_types.len(), 1, "should have 1 type ban");
        assert_eq!(cfg.disallowed_macros.len(), 2, "should have 2 macro bans");
    }

    #[test]
    fn detailed_ban_entries_with_reason() {
        let cfg = parse(r#"
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    "std::process::exit",
]
"#);

        assert_eq!(cfg.disallowed_methods.len(), 2, "should have 2 entries");
        assert_eq!(cfg.disallowed_methods[0].path(), "std::env::var", "detailed path");
        assert_eq!(cfg.disallowed_methods[0].reason(), Some("Use config module"), "detailed reason");
        assert_eq!(cfg.disallowed_methods[1].path(), "std::process::exit", "simple path");
        assert_eq!(cfg.disallowed_methods[1].reason(), None, "simple has no reason");
    }

    #[test]
    fn test_relaxations_parse() {
        let cfg = parse(r#"
allow-dbg-in-tests = false
allow-print-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-unwrap-in-tests = false
"#);

        assert_eq!(cfg.allow_dbg_in_tests, Some(false), "allow_dbg mismatch");
        assert_eq!(cfg.allow_print_in_tests, Some(false), "allow_print mismatch");
        assert_eq!(cfg.allow_expect_in_tests, Some(true), "allow_expect mismatch");
        assert_eq!(cfg.allow_panic_in_tests, Some(false), "allow_panic mismatch");
        assert_eq!(cfg.allow_unwrap_in_tests, Some(false), "allow_unwrap mismatch");
    }

    #[test]
    fn unknown_keys_land_in_extra() {
        let cfg = parse(r#"
max-struct-bools = 3
some-future-clippy-option = "yes"
"#);

        assert_eq!(cfg.max_struct_bools, Some(3), "known key should parse");
        assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown key");
        assert_eq!(
            cfg.extra.get("some-future-clippy-option").and_then(Value::as_str),
            Some("yes"),
            "unknown key should be captured",
        );
    }

    #[test]
    fn real_clippy_toml_parses() {
        let cfg = parse(r#"
too-many-lines-threshold = 75
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 7
type-complexity-threshold = 75
max-struct-bools = 3
max-fn-params-bools = 3
excessive-nesting-threshold = 4
avoid-breaking-exported-api = false
allow-dbg-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-print-in-tests = false
allow-unwrap-in-tests = false
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    { path = "std::process::exit", reason = "Use error propagation" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "Use BTreeMap" },
]
disallowed-macros = [
    { path = "println", reason = "Use tracing" },
]
"#);

        assert_eq!(cfg.max_struct_bools, Some(3), "threshold mismatch");
        assert_eq!(cfg.avoid_breaking_exported_api, Some(false), "api flag mismatch");
        assert_eq!(cfg.disallowed_methods.len(), 2, "method ban count");
        assert_eq!(cfg.disallowed_types.len(), 1, "type ban count");
        assert_eq!(cfg.disallowed_macros.len(), 1, "macro ban count");
        assert!(cfg.extra.is_empty(), "all keys should be known");
    }

    #[test]
    fn from_str_error_on_invalid_toml() {
        let bad = "this is not [[[valid toml";
        let err = bad.parse::<ClippyConfig>();
        assert!(err.is_err(), "invalid TOML should produce an error");

        let msg = err.expect_err("should be an error").to_string();
        assert!(
            msg.contains("invalid clippy.toml"),
            "expected error message prefix, got: {msg}",
        );
    }
}
