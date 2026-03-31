mod discover;
mod facts;
mod inputs;
mod rs_toolchain_01_exists;
mod rs_toolchain_02_channel_and_components;
mod rs_toolchain_03_msrv_consistency;
mod rs_toolchain_04_legacy_file;

use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

use self::discover::collect;
use self::inputs::all_from_facts;

pub fn check(tree: &ProjectTree, route: &RsToolchainRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in all_from_facts(&facts) {
        rs_toolchain_01_exists::check(&input, &mut results);
        rs_toolchain_02_channel_and_components::check(&input, &mut results);
        rs_toolchain_03_msrv_consistency::check(&input, &mut results);
        rs_toolchain_04_legacy_file::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Toolchain]));
    let route =
        FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_toolchain();
    check(tree, &route)
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
