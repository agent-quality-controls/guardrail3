use std::path::Path;

use guardrail3_app_rs_family_test as runtime;
use guardrail3_domain_report::{CheckResult, Severity};
use test_support::{
    StubToolChecker, finding as finding_for_rule, rule_files as rule_files_for_rule, walk,
};

const RULE_ID: &str = "RS-TEST-11";

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk(root);
    runtime::check(&tree, &StubToolChecker::default(), None)
}

pub fn run_family_with_tool(root: &Path, cargo_mutants_installed: bool) -> Vec<CheckResult> {
    let tree = walk(root);
    let checker = if cargo_mutants_installed {
        StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        StubToolChecker::default()
    };
    runtime::check(&tree, &checker, None)
}

pub fn rule_files(results: &[CheckResult], _rule_id: &str) -> Vec<String> {
    rule_files_for_rule(results, RULE_ID)
}

pub fn finding<'a>(results: &'a [CheckResult], _rule_id: &str) -> &'a CheckResult {
    finding_for_rule(results, RULE_ID)
}

pub fn assert_rule_quiet(results: &[CheckResult]) {
    assert!(
        rule_files_for_rule(results, RULE_ID).is_empty(),
        "expected no {RULE_ID} findings"
    );
}

pub fn assert_finding_matches(
    results: &[CheckResult],
    file: &str,
    line: Option<usize>,
    severity: Severity,
    title: &str,
) {
    let finding = finding_for_rule(results, RULE_ID);
    assert_eq!(finding.severity, severity);
    assert_eq!(finding.title, title);
    assert_eq!(finding.file.as_deref(), Some(file));
    assert_eq!(finding.line, line);
}
