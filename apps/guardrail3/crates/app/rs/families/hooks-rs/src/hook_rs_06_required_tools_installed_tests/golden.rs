use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_06_required_tools_installed as assertions;

use crate::hook_rs_06_required_tools_installed::run_case;

#[test]
fn reports_all_required_tools_as_inventory_when_installed() {
    let results = run_case(&["gitleaks", "cargo-deny", "cargo-machete"]);
    assert_eq!(results.len(), 3);
    assertions::assert_tool_present(&results, "gitleaks");
    assertions::assert_tool_present(&results, "cargo-deny");
    assertions::assert_tool_present(&results, "cargo-machete");
}

#[test]
fn reports_missing_tool_as_error() {
    let results = run_case(&["gitleaks", "cargo-deny"]);
    assert_eq!(results.len(), 3);
    assertions::assert_tool_missing(&results, "cargo-machete");
}

#[test]
fn reports_gitleaks_missing_as_error() {
    let results = run_case(&["cargo-deny", "cargo-machete"]);
    assertions::assert_tool_missing(&results, "gitleaks");
}

#[test]
fn reports_cargo_deny_missing_as_error() {
    let results = run_case(&["gitleaks", "cargo-machete"]);
    assertions::assert_tool_missing(&results, "cargo-deny");
}

#[test]
fn reports_all_tools_missing_as_distinct_errors() {
    let results = run_case(&[]);
    assert_eq!(results.len(), 3);
    assertions::assert_tool_missing(&results, "gitleaks");
    assertions::assert_tool_missing(&results, "cargo-deny");
    assertions::assert_tool_missing(&results, "cargo-machete");
}

#[test]
fn reports_mixed_installed_and_missing_tools_in_same_run() {
    let results = run_case(&["gitleaks"]);
    assert_eq!(results.len(), 3);
    assertions::assert_tool_present(&results, "gitleaks");
    assertions::assert_tool_missing(&results, "cargo-deny");
    assertions::assert_tool_missing(&results, "cargo-machete");
}

#[test]
fn only_reports_the_three_expected_tool_names() {
    let results = run_case(&[
        "gitleaks",
        "cargo-deny",
        "cargo-machete",
        "guardrail3",
        "cargo-dupes",
    ]);
    assert_eq!(results.len(), 3);
    assertions::assert_tool_present(&results, "gitleaks");
    assertions::assert_tool_present(&results, "cargo-deny");
    assertions::assert_tool_present(&results, "cargo-machete");
}
