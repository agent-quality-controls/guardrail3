//! Typed document model for `.jscpd.json` configurations.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Parsed `.jscpd.json` document carrying both the raw JSON value and the
/// typed parse state.
///
/// The `Document` suffix mirrors the public type name shape used by the
/// other parser facades in this repo (one parsed document per file).
#[allow(
    clippy::module_name_repetitions,
    reason = "the `Document` suffix is the established public naming for parser facades in this repo"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JscpdDocument {
    /// Raw JSON value as parsed by `serde_json`.
    pub raw: Value,
    /// Typed parse state (success or failure with a reason).
    pub typed: JscpdParseState,
}

/// Outcome of attempting to deserialize the typed `JscpdSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JscpdParseState {
    /// Successful typed parse.
    Parsed(JscpdSnapshot),
    /// Typed parse failed; carries the human-readable reason.
    Invalid(String),
}

/// Typed view of the configuration fields recognised in `.jscpd.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JscpdSnapshot {
    /// Duplication-percentage threshold above which the run fails.
    #[serde(default)]
    pub threshold: Option<i64>,
    /// Minimum-tokens threshold below which clones are ignored.
    #[serde(rename = "minTokens", default)]
    pub min_tokens: Option<u64>,
    /// Whether absolute clone counts are reported alongside percentages.
    #[serde(default)]
    pub absolute: Option<bool>,
    /// Languages or formats included in the scan.
    #[serde(default)]
    pub format: Vec<String>,
    /// Path patterns excluded from the scan.
    #[serde(default)]
    pub ignore: Vec<String>,
    /// Additional configuration keys callers explicitly pass through.
    #[serde(default = "default_extra_keys")]
    pub extra_keys: Vec<String>,
}

/// Default value for [`JscpdSnapshot::extra_keys`] (empty list).
const fn default_extra_keys() -> Vec<String> {
    Vec::new()
}
