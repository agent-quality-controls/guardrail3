mod api_shape;
mod cfg_and_paths;
mod discover;
mod facts;
mod hygiene;
mod inputs;
mod inventory;
mod lint_policy;
mod parse;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_adapters_outbound_fs::RealFileSystem;
#[cfg(test)]
use guardrail3_app_core::project_walker::walk_project;
#[cfg(test)]
use guardrail3_app_rs_family_code_assertions as _;

#[cfg(test)]
const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/full_golden";

#[cfg(test)]
#[must_use]
pub(crate) fn check_test_root(
    root: &std::path::Path,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    let surface = guardrail3_app_rs_family_view::FamilyView::build(
        tree.root().clone(),
        tree.structure(),
        tree.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    );
    check_test_tree(&surface)
}

#[cfg(test)]
#[must_use]
pub(crate) fn check_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    check(
        tree,
        &family_route_for_tests(tree),
    )
}

#[cfg(test)]
fn family_route_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsCodeRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = parse_guardrail_config(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Code,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(
        &legality,
        config.as_ref(),
        &selected,
        None,
    )
    .map_rs_code()
}

#[cfg(test)]
fn parse_guardrail_config(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Option<guardrail3_domain_config::types::GuardrailConfig> {
    tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    })
}

#[cfg(test)]
pub(crate) fn copy_test_fixture() -> test_support::TempDir {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL);
    test_support::copy_tree(&root)
}
