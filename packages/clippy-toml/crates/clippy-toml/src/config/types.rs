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
            Self::Detailed(detail) => detail.path(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed ban entry with path and optional reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BanEntryDetail {
    /// The banned path (e.g., `"std::env::var"`).
    path: String,
    /// Why this item is banned.
    reason: Option<String>,
    /// Catch-all for additional fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanEntryDetail {
    /// The banned path (e.g., `"std::env::var"`).
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Why this item is banned, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
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
