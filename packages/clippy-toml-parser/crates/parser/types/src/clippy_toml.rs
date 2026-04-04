use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Parsed representation of a `clippy.toml` / `.clippy.toml` configuration file.
///
/// All known Clippy configuration keys are mapped to typed fields.
/// Unknown keys are captured in [`extra`](Self::extra) for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)] // reason: clippy.toml has 10+ boolean configuration options — struct mirrors the schema
pub struct ClippyToml {
    pub max_struct_bools: Option<u32>,
    pub max_fn_params_bools: Option<u32>,
    pub too_many_lines_threshold: Option<u32>,
    pub too_many_arguments_threshold: Option<u32>,
    pub excessive_nesting_threshold: Option<u32>,
    pub cognitive_complexity_threshold: Option<u32>,
    pub type_complexity_threshold: Option<u32>,
    #[serde(default)]
    pub disallowed_methods: Vec<BanEntry>,
    #[serde(default)]
    pub disallowed_types: Vec<BanEntry>,
    #[serde(default)]
    pub disallowed_macros: Vec<BanEntry>,
    pub allow_dbg_in_tests: Option<bool>,
    pub allow_print_in_tests: Option<bool>,
    pub allow_expect_in_tests: Option<bool>,
    pub allow_panic_in_tests: Option<bool>,
    pub allow_unwrap_in_tests: Option<bool>,
    pub avoid_breaking_exported_api: Option<bool>,
    pub msrv: Option<String>,
    pub too_large_for_stack: Option<u64>,
    #[serde(default)]
    pub absolute_paths_allowed_crates: Vec<String>,
    pub absolute_paths_max_segments: Option<u32>,
    #[serde(default)]
    pub ignore_interior_mutability: Vec<String>,
    #[serde(default)]
    pub await_holding_invalid_types: Vec<Value>,
    #[serde(default)]
    pub arithmetic_side_effects_allowed: Vec<Value>,
    pub lint_commented_code: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A ban entry in `disallowed-methods`, `disallowed-types`, or `disallowed-macros`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanEntry {
    Simple(String),
    Detailed(BanEntryDetail),
}

impl BanEntry {
    #[must_use]
    pub fn path(&self) -> &str {
        match self {
            Self::Simple(path) => path,
            Self::Detailed(detail) => detail.path(),
        }
    }

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
    /// The banned path.
    path: String,
    /// Why this item is banned.
    reason: Option<String>,
    /// Additional unmapped fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanEntryDetail {
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}
