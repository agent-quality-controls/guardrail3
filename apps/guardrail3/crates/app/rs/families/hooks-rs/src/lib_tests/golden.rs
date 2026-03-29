use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_14_guardrail_binary_available as hook_rs_14_assertions;
use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_15_cargo_dupes_installed as hook_rs_15_assertions;
use super::super::run_case;

#[test]
fn orchestrator_skips_hook_rs_14_when_guardrail_validation_is_not_expected() {
    let results = run_case("echo noop\n", &[]);
    hook_rs_14_assertions::assert_rule_quiet(&results);
}

#[test]
fn orchestrator_marks_hook_rs_14_present_for_path_qualified_guardrail_validation() {
    let results = run_case("/usr/local/bin/guardrail3 rs validate --staged .\n", &[]);
    hook_rs_14_assertions::assert_present(&results);
}

#[test]
fn orchestrator_skips_hook_rs_15_when_cargo_dupes_is_not_required() {
    let results = run_case("echo noop\n", &[]);
    hook_rs_15_assertions::assert_rule_quiet(&results);
}

#[test]
fn orchestrator_marks_hook_rs_15_present_for_path_qualified_cargo_dupes() {
    let results = run_case("/usr/local/bin/cargo-dupes check --exclude-tests\n", &[]);
    hook_rs_15_assertions::assert_present(&results);
}

#[test]
fn orchestrator_marks_hook_rs_15_present_for_wrapped_path_qualified_cargo_dupes() {
    let results = run_case("exec /usr/local/bin/cargo-dupes check --exclude-tests\n", &[]);
    hook_rs_15_assertions::assert_present(&results);
}

#[test]
fn orchestrator_does_not_treat_wrapper_prose_as_path_qualified_cargo_dupes() {
    let results = run_case(
        "cargo dupes check --exclude-tests\nbash -lc 'echo /usr/local/bin/cargo-dupes check --exclude-tests'\n",
        &[],
    );
    hook_rs_15_assertions::assert_missing(&results);
}
