mod discover;
mod facts;
mod inputs;
mod rs_toolchain_01_exists;
mod rs_toolchain_02_channel_and_components;
mod rs_toolchain_03_msrv_consistency;
mod rs_toolchain_04_legacy_file;

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
pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Toolchain]));
    let route =
        FamilyMapper::from_legality(&legality, config.as_ref(), &selected, None).map_rs_toolchain();
    check(tree, &route)
}

#[cfg(test)]

mod tests;
