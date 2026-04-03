use super::super::{check, check_parseable};
use guardrail3_domain_report::CheckResult;
use crate::inputs::PolicyContextFailureInput;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
pub(super) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = crate::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
        check(&PolicyContextFailureInput::new(parse_error), &mut results);
    } else if tree.file_exists("guardrail3.toml") {
        check_parseable(&mut results);
    }
    results
}
