// reason: dev-dep retained per `g3rs-test/runtime-assertions-split` contract; the runtime
// is exercised both as the lib under test and as a dev-dep transitive through the
// assertions crate, so cargo creates two distinct crate units. Calling assertion helpers
// from runtime unit tests would mix those units and produce type-identity mismatches, so
// these tests assert directly against the runtime types.
use guardrail3_reason_policy_assertions as _;

use super::super::reason_text_is_useful;
use super::super::validate_reason_text;
use super::super::{DEFAULT_MIN_REASON_CHARS, DEFAULT_MIN_REASON_WORDS, ReasonIssue};

#[test]
fn rejects_short_reasons() {
    assert_eq!(
        validate_reason_text("two words"),
        Err(ReasonIssue::TooShort {
            min_chars: DEFAULT_MIN_REASON_CHARS,
            actual_chars: 9,
        }),
        "short reasons must be rejected by the default policy",
    );
    assert!(
        !reason_text_is_useful("two words"),
        "short reasons must not be considered useful",
    );
}

#[test]
fn rejects_single_word_reasons_even_when_long_enough() {
    assert_eq!(
        validate_reason_text("compatibility"),
        Err(ReasonIssue::TooFewWords {
            min_words: DEFAULT_MIN_REASON_WORDS,
            actual_words: 1,
        }),
        "single-word reasons must be rejected even when they meet the character threshold",
    );
}

#[test]
fn rejects_empty_reasons() {
    assert_eq!(
        validate_reason_text("   "),
        Err(ReasonIssue::Empty),
        "whitespace-only reasons must be rejected as empty",
    );
}

#[test]
fn rejects_placeholder_reasons() {
    assert_eq!(
        validate_reason_text("temp"),
        Err(ReasonIssue::Placeholder),
        "placeholder reasons must be rejected",
    );
    assert_eq!(
        validate_reason_text("fix later"),
        Err(ReasonIssue::Placeholder),
        "placeholder reasons must be rejected",
    );
}

#[test]
fn accepts_nontrivial_reasons() {
    assert_eq!(
        validate_reason_text("compatibility shim"),
        Ok(()),
        "non-trivial multi-word reasons must be accepted",
    );
    assert!(
        reason_text_is_useful("outer workflow validation"),
        "non-trivial multi-word reasons must be considered useful",
    );
}
