use super::super::{check, check_allowed};
use guardrail3_domain_report::CheckResult;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
pub(super) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = crate::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    for allowed in &facts.allowed_configs {
        check_allowed(allowed, &mut results);
    }
    for forbidden in &facts.forbidden_configs {
        check(forbidden, &mut results);
    }
    results
}
pub(super) fn run_with_validation_scope_for_tests(
    tree: &ProjectTree,
    validation_scope: &str,
) -> Vec<CheckResult> {
    let facts = crate::facts::collect_with_validation_scope_for_tests(tree, validation_scope);
    let mut results = Vec::new();
    for allowed in &facts.allowed_configs {
        check_allowed(allowed, &mut results);
    }
    for forbidden in &facts.forbidden_configs {
        check(forbidden, &mut results);
    }
    results
}
