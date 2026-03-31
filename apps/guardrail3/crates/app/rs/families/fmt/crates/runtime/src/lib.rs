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

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::facts::{collect, file_name_kind};
use self::inputs::{RustfmtDualConflictInput, RustfmtExtraConfigInput, RustfmtRootInput};

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

pub fn check(tree: &ProjectTree, route: &guardrail3_app_rs_family_mapper::RsFmtRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    let root = RustfmtRootInput::from_facts(&facts);
    rs_fmt_01_exists::check(&root, &mut results);
    rs_fmt_02_settings::check(&root, &mut results);
    rs_fmt_03_extra_settings::check(&root, &mut results);
    rs_fmt_04_nightly_keys_on_stable::check(&root, &mut results);
    rs_fmt_06_edition_mismatch::check(&root, &mut results);
    rs_fmt_07_ignore_escape_hatch::check(&root, &mut results);

    for config_rel in &facts.extra_config_rels {
        let input = RustfmtExtraConfigInput {
            config_rel: config_rel.clone(),
            config_kind: file_name_kind(config_rel),
        };
        rs_fmt_05_per_crate_override::check(&input, &mut results);
    }

    for dir_rel in &facts.dual_file_conflict_dirs {
        let input = RustfmtDualConflictInput {
            dir_rel: dir_rel.clone(),
        };
        rs_fmt_08_dual_file_conflict::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
pub(crate) fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Fmt]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_fmt();
    check(tree, &route)
}

#[cfg(test)]
pub(crate) fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    check_test_tree(&tree)
}
