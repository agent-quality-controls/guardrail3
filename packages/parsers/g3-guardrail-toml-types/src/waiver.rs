use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// One exact waiver target requested by a rule before it downgrades a finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaiverMatch<'a> {
    /// Semantic rule id requested by the caller.
    rule: &'a str,
    /// Repo-relative file path requested by the caller.
    file: &'a str,
    /// Rule-local selector requested by the caller.
    selector: &'a str,
}

impl<'a> WaiverMatch<'a> {
    /// Builds an exact waiver target.
    #[must_use]
    pub const fn new(rule: &'a str, file: &'a str, selector: &'a str) -> Self {
        Self {
            rule,
            file,
            selector,
        }
    }
}

/// Non-empty reason attached to a matching waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaiverReason<'a>(&'a str);

impl<'a> WaiverReason<'a> {
    /// Returns the configured waiver reason.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.0
    }
}

/// Shared representation of one `[[waivers]]` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WaiverConfig {
    /// Semantic rule id, for example `g3rs-deps/direct-dependency-cap`.
    pub rule: String,
    /// Repo-relative file path reported by the rule.
    pub file: String,
    /// Rule-specific selector for the exact bypassed assertion.
    pub selector: String,
    /// Human-readable reason explaining why this escape hatch exists.
    pub reason: String,
    /// Forward-compatible storage for future waiver fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Returns the reason for the first exact matching waiver with a non-empty reason.
#[must_use]
pub fn find_waiver_reason<'a>(
    waivers: &'a [WaiverConfig],
    target: &WaiverMatch<'_>,
) -> Option<WaiverReason<'a>> {
    waivers
        .iter()
        .find(|waiver| matches_target(waiver, target))
        .and_then(valid_reason)
}

/// Returns whether a non-empty exact matching waiver exists.
#[must_use]
pub fn has_waiver(waivers: &[WaiverConfig], target: &WaiverMatch<'_>) -> bool {
    find_waiver_reason(waivers, target).is_some()
}

/// Checks exact rule, file, and selector equality.
fn matches_target(waiver: &WaiverConfig, target: &WaiverMatch<'_>) -> bool {
    waiver.rule == target.rule && waiver.file == target.file && waiver.selector == target.selector
}

/// Returns a reason only when it has non-whitespace content.
fn valid_reason(waiver: &WaiverConfig) -> Option<WaiverReason<'_>> {
    if waiver.reason.trim().is_empty() {
        return None;
    }
    Some(WaiverReason(waiver.reason.as_str()))
}
