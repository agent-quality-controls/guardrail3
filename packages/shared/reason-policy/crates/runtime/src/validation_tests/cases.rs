use guardrail3_reason_policy_assertions::validation as assertions;

use super::super::reason_text_is_useful;
use super::super::validate_reason_text;

#[test]
fn rejects_short_reasons() {
    assertions::assert_rejects_short_reason(validate_reason_text("two words"));
    assertions::assert_is_not_useful(reason_text_is_useful("two words"));
}

#[test]
fn rejects_single_word_reasons_even_when_long_enough() {
    assertions::assert_rejects_single_word_reason(validate_reason_text("compatibility"));
}

#[test]
fn rejects_empty_reasons() {
    assertions::assert_rejects_empty_reason(validate_reason_text("   "));
}

#[test]
fn rejects_placeholder_reasons() {
    assertions::assert_rejects_placeholder_reason(validate_reason_text("temp"));
    assertions::assert_rejects_placeholder_reason(validate_reason_text("fix later"));
}

#[test]
fn accepts_nontrivial_reasons() {
    assertions::assert_accepts_useful_reason(validate_reason_text("compatibility shim"));
    assertions::assert_is_useful(reason_text_is_useful("outer workflow validation"));
}
