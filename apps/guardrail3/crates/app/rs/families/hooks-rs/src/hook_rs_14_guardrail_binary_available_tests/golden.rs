use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_14_guardrail_binary_available as assertions;

use crate::hook_rs_14_guardrail_binary_available::run_case;

#[test]
fn passes_when_guardrail_binary_is_installed() {
    let results = run_case(true, false, &["guardrail3"]);
    assert_eq!(results.len(), 1);
    assertions::assert_present(&results);
}

#[test]
fn errors_when_guardrail_binary_is_missing() {
    let results = run_case(true, false, &[]);
    assertions::assert_missing(&results);
}

#[test]
fn skips_when_guardrail_validation_is_not_expected() {
    let results = run_case(false, false, &[]);
    assert!(results.is_empty());
}

#[test]
fn passes_when_guardrail_validation_uses_explicit_path() {
    let results = run_case(true, true, &[]);
    assertions::assert_present(&results);
}
