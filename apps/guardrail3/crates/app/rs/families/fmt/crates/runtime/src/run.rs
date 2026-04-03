use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::facts::{collect, file_name_kind};
use crate::inputs::{RustfmtDualConflictInput, RustfmtExtraConfigInput, RustfmtRootInput};

pub fn check(
    surface: &FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsFmtRoute,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    let root = RustfmtRootInput::from_facts(&facts);
    crate::rs_fmt_01_exists::check(&root, &mut results);
    crate::rs_fmt_02_settings::check(&root, &mut results);
    crate::rs_fmt_03_extra_settings::check(&root, &mut results);
    crate::rs_fmt_04_nightly_keys_on_stable::check(&root, &mut results);
    crate::rs_fmt_06_edition_mismatch::check(&root, &mut results);
    crate::rs_fmt_07_ignore_escape_hatch::check(&root, &mut results);

    for config_rel in &facts.extra_config_rels {
        let input = RustfmtExtraConfigInput {
            config_rel: config_rel.clone(),
            config_kind: file_name_kind(config_rel),
        };
        crate::rs_fmt_05_per_crate_override::check(&input, &mut results);
    }

    for dir_rel in &facts.dual_file_conflict_dirs {
        let input = RustfmtDualConflictInput {
            dir_rel: dir_rel.clone(),
        };
        crate::rs_fmt_08_dual_file_conflict::check(&input, &mut results);
    }

    results
}
