use crate::ReasonIssue;
use crate::ReasonPolicy;

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

pub fn validate_reason_text(reason: &str) -> Result<(), ReasonIssue> {
    validate_reason_text_with_policy(reason, ReasonPolicy::default())
}

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

#[must_use]
pub fn reason_text_is_useful(reason: &str) -> bool {
    validate_reason_text(reason).is_ok()
}

#[cfg(test)]
#[path = "validation_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod validation_tests;
