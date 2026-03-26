use guardrail3_app_rs_family_cargo as runtime;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

pub(crate) fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    runtime::check_test_tree(tree)
}

pub(crate) fn rule_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results.iter().filter(|result| result.id == id).collect()
}

pub(crate) fn has_result<F>(results: &[CheckResult], id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    results
        .iter()
        .any(|result| result.id == id && predicate(result))
}
