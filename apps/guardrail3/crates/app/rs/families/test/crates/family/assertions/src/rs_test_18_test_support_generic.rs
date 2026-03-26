use std::path::Path;

use guardrail3_app_rs_family_test as runtime;
use guardrail3_domain_report::CheckResult;
use test_support::{
    StubToolChecker, finding as finding_for_rule, rule_files as rule_files_for_rule, walk,
};

const RULE_ID: &str = "RS-TEST-18";

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = walk(root);
    runtime::check(&tree, &StubToolChecker::default(), None)
}

pub fn rule_files(results: &[CheckResult], _rule_id: &str) -> Vec<String> {
    rule_files_for_rule(results, RULE_ID)
}

pub fn finding<'a>(results: &'a [CheckResult], _rule_id: &str) -> &'a CheckResult {
    finding_for_rule(results, RULE_ID)
}
