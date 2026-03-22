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

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::{collect, file_name_kind};
use self::inputs::{RustfmtDualConflictInput, RustfmtExtraConfigInput, RustfmtRootInput};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
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
            config_rel,
            config_kind: file_name_kind(config_rel),
        };
        rs_fmt_05_per_crate_override::check(&input, &mut results);
    }

    for dir_rel in &facts.dual_file_conflict_dirs {
        let input = RustfmtDualConflictInput { dir_rel };
        rs_fmt_08_dual_file_conflict::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "fmt_tests.rs"]
mod tests;
