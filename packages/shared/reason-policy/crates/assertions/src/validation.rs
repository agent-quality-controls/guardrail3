#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_reason_policy_runtime::DEFAULT_MIN_REASON_CHARS;
use guardrail3_reason_policy_runtime::DEFAULT_MIN_REASON_WORDS;
use guardrail3_reason_policy_runtime::ReasonIssue;

/// Generates an assertion helper that compares a `validate_reason_text` result
/// against a fixed expected outcome.
macro_rules! reject_assertion {
    ($name:ident, $expected:expr, $msg:literal) => {
        #[doc = concat!("Asserts that `result` is the expected ", $msg, " rejection.")]
        pub fn $name(result: Result<(), ReasonIssue>) {
            assert_eq!(result, $expected, $msg);
        }
    };
}

reject_assertion!(
    assert_rejects_short_reason,
    Err(ReasonIssue::TooShort {
        min_chars: DEFAULT_MIN_REASON_CHARS,
        actual_chars: 9,
    }),
    "9-character reason must be rejected as TooShort under the default policy"
);

reject_assertion!(
    assert_rejects_single_word_reason,
    Err(ReasonIssue::TooFewWords {
        min_words: DEFAULT_MIN_REASON_WORDS,
        actual_words: 1,
    }),
    "single-word reason must be rejected as TooFewWords under the default policy"
);

reject_assertion!(
    assert_rejects_empty_reason,
    Err(ReasonIssue::Empty),
    "whitespace-only reason must be rejected as Empty"
);

reject_assertion!(
    assert_rejects_placeholder_reason,
    Err(ReasonIssue::Placeholder),
    "placeholder reason must be rejected as Placeholder"
);

reject_assertion!(
    assert_accepts_useful_reason,
    Ok(()),
    "non-trivial reason must be accepted by the default policy"
);

/// Asserts that `useful` is `false`.
pub fn assert_is_not_useful(useful: bool) {
    assert!(!useful, "reason should be rejected");
}

/// Asserts that `useful` is `true`.
pub fn assert_is_useful(useful: bool) {
    assert!(useful, "reason should be accepted");
}
