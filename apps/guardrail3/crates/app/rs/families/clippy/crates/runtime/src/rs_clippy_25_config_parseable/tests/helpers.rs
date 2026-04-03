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
pub(super) fn run_family_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Clippy,
        ]));
    let route =
        guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
            .map_rs_clippy();
    crate::check(tree, &route)
}
