use guardrail3_check_types::G3CheckResult;
use serde::{Deserialize, Serialize};

/// Exact key that identifies one waiverable finding instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaiverKey<'a> {
    /// Semantic rule id, for example `g3rs-deps/direct-dependency-cap`.
    rule: &'a str,
    /// Repo-relative subject reported by the finding.
    subject: &'a str,
    /// Rule-specific selector for the exact assertion.
    selector: &'a str,
}

impl<'a> WaiverKey<'a> {
    /// Builds an exact waiver key.
    #[must_use]
    pub const fn new(rule: &'a str, subject: &'a str, selector: &'a str) -> Self {
        Self {
            rule,
            subject,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaiverConfig {
    /// Semantic rule id, for example `g3rs-deps/direct-dependency-cap`.
    rule: String,
    /// Repo-relative finding subject.
    subject: String,
    /// Rule-specific selector for the exact bypassed assertion.
    selector: String,
    /// Human-readable reason explaining why this escape hatch exists.
    reason: String,
}

/// Applies matching waivers to result severities and messages.
pub fn apply_waivers(results: &mut [G3CheckResult], waivers: &[WaiverConfig]) {
    for result in results {
        let (rule, subject, selector) = result.waiver_key();
        let key = WaiverKey::new(rule, subject, selector);
        if let Some(reason) = find_waiver_reason(waivers, &key) {
            result.apply_waiver(reason.as_str());
        }
    }
}

/// Returns the reason for the first exact matching waiver with a non-empty reason.
#[must_use]
pub fn find_waiver_reason<'a>(
    waivers: &'a [WaiverConfig],
    target: &WaiverKey<'_>,
) -> Option<WaiverReason<'a>> {
    waivers
        .iter()
        .find(|waiver| matches_target(waiver, target))
        .and_then(valid_reason)
}

/// Checks exact rule, subject, and selector equality.
fn matches_target(waiver: &WaiverConfig, target: &WaiverKey<'_>) -> bool {
    waiver.rule == target.rule
        && waiver.subject == target.subject
        && waiver.selector == target.selector
}

/// Returns a reason only when it has non-whitespace content.
fn valid_reason(waiver: &WaiverConfig) -> Option<WaiverReason<'_>> {
    if waiver.reason.trim().is_empty() {
        return None;
    }
    Some(WaiverReason(waiver.reason.as_str()))
}
