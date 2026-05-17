use crate::ReasonIssue;
use crate::ReasonPolicy;

/// Lowercased phrases treated as non-informative reason placeholders.
const PLACEHOLDERS: &[&str] = &[
    "ok",
    "okay",
    "present",
    "works",
    "valid",
    "value",
    "error",
    "failed",
    "failure",
    "test",
    "reason",
    "tbd",
    "todo",
    "fixme",
    "temp",
    "temporary",
    "legacy",
    "fix later",
    "...",
];

/// Validates `reason` against the default reason policy.
///
/// # Errors
/// Returns the first [`ReasonIssue`] detected in `reason` under the default policy.
pub fn validate_reason_text(reason: &str) -> Result<(), ReasonIssue> {
    validate_reason_text_with_policy(reason, ReasonPolicy::default())
}

/// Validates `reason` against the supplied [`ReasonPolicy`].
///
/// # Errors
/// Returns the first [`ReasonIssue`] detected in `reason` under `policy`.
pub fn validate_reason_text_with_policy(
    reason: &str,
    policy: ReasonPolicy,
) -> Result<(), ReasonIssue> {
    let trimmed = reason.trim();
    if trimmed.is_empty() {
        return Err(ReasonIssue::Empty);
    }

    let normalized = trimmed.to_ascii_lowercase();
    if PLACEHOLDERS.contains(&normalized.as_str()) {
        return Err(ReasonIssue::Placeholder);
    }

    let actual_chars = trimmed.chars().count();
    if actual_chars < policy.min_chars() {
        return Err(ReasonIssue::TooShort {
            min_chars: policy.min_chars(),
            actual_chars,
        });
    }

    let actual_words = trimmed.split_whitespace().count();
    if actual_words < policy.min_words() {
        return Err(ReasonIssue::TooFewWords {
            min_words: policy.min_words(),
            actual_words,
        });
    }

    Ok(())
}

/// Returns `true` when `reason` passes the default reason policy.
#[must_use]
pub fn reason_text_is_useful(reason: &str) -> bool {
    validate_reason_text(reason).is_ok()
}
