mod facts;
mod inputs;
mod rs_fmt_01_exists;
mod rs_fmt_02_settings;
mod rs_fmt_03_extra_settings;
mod rs_fmt_04_nightly_keys_on_stable;
mod rs_fmt_05_per_crate_override;
mod rs_fmt_06_edition_mismatch;
mod rs_fmt_07_ignore_escape_hatch;
mod rs_fmt_08_dual_file_conflict;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use tempfile as _;

#[cfg(test)]
pub(crate) fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Fmt]));
    let route = FamilyMapper::from_legality(&legality, config.as_ref(), &selected, None).map_rs_fmt();
    check(tree, &route)
}

#[cfg(test)]
pub(crate) fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    check_test_tree(&tree)
}
