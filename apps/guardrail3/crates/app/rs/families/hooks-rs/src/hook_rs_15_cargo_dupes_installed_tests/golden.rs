use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_15_cargo_dupes_installed as assertions;

use crate::hook_rs_15_cargo_dupes_installed::run_case;

#[test]
fn passes_when_cargo_dupes_is_installed() {
    let results = run_case(true, false, &["cargo-dupes"]);
    assert_eq!(results.len(), 1);
    assertions::assert_present(&results);
}

#[test]
fn errors_when_cargo_dupes_is_missing() {
    let results = run_case(true, false, &[]);
    assertions::assert_missing(&results);
}

#[test]
fn skips_when_cargo_dupes_is_not_required() {
    let results = run_case(false, false, &[]);
    assert!(results.is_empty());
}

#[test]
fn passes_when_cargo_dupes_uses_explicit_path() {
    let results = run_case(true, true, &[]);
    assertions::assert_present(&results);
}
