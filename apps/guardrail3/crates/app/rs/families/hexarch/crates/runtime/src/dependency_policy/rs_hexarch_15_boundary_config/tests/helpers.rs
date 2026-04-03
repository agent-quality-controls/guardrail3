use super::super::{check};
use guardrail3_domain_report::CheckResult;
use crate::inputs::MemberConfigHexarchInput;
pub fn check_boundary_config_for_test(
    rel_dir: &str,
    has_config_entry: bool,
    is_app_boundary: bool,
    parse_error: Option<&str>,
) -> Vec<CheckResult> {
    let input = crate::dependency_facts::BoundaryConfigFacts {
        rel_dir: rel_dir.to_owned(),
        has_config_entry,
        is_app_boundary,
        parse_error: parse_error.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&MemberConfigHexarchInput::new(&input), &mut results);
    results
}
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
pub(super) fn results_for_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}
