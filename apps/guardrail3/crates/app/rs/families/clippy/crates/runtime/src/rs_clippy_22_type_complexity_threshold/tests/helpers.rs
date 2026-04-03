use super::super::{check};
use guardrail3_domain_report::CheckResult;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
pub(super) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = crate::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(
        &crate::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}
