#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_reason_policy_runtime::DEFAULT_MIN_REASON_CHARS;
use guardrail3_reason_policy_runtime::DEFAULT_MIN_REASON_WORDS;
use guardrail3_reason_policy_runtime::ReasonIssue;

pub fn assert_rejects_short_reason(result: Result<(), ReasonIssue>) {
    assert_eq!(
        result,
        Err(ReasonIssue::TooShort {
            min_chars: DEFAULT_MIN_REASON_CHARS,
            actual_chars: 9,
        })
    );
}

pub fn assert_rejects_single_word_reason(result: Result<(), ReasonIssue>) {
    assert_eq!(
        result,
        Err(ReasonIssue::TooFewWords {
            min_words: DEFAULT_MIN_REASON_WORDS,
            actual_words: 1,
        })
    );
}

pub fn assert_rejects_empty_reason(result: Result<(), ReasonIssue>) {
    assert_eq!(result, Err(ReasonIssue::Empty));
}

pub fn assert_rejects_placeholder_reason(result: Result<(), ReasonIssue>) {
    assert_eq!(result, Err(ReasonIssue::Placeholder));
}

pub fn assert_accepts_useful_reason(result: Result<(), ReasonIssue>) {
    assert_eq!(result, Ok(()));
}

pub fn assert_is_not_useful(useful: bool) {
    assert!(!useful, "reason should be rejected");
}

pub fn assert_is_useful(useful: bool) {
    assert!(useful, "reason should be accepted");
}
