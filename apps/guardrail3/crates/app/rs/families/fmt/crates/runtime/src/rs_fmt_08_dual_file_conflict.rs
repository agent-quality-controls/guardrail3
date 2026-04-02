use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtDualConflictInput;

const ID: &str = "RS-FMT-08";

pub fn check(input: &RustfmtDualConflictInput, results: &mut Vec<CheckResult>) {
    let file = if input.dir_rel.is_empty() {
        "rustfmt.toml".to_owned()
    } else {
        ProjectTree::join_rel(&input.dir_rel, "rustfmt.toml")
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "Conflicting rustfmt config files".to_owned(),
        "Both rustfmt.toml and .rustfmt.toml exist in the same directory".to_owned(),
        Some(file),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_check(dir_rel: &str) -> Vec<CheckResult> {
    let input = RustfmtDualConflictInput {
        dir_rel: dir_rel.to_owned(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_fmt_08_dual_file_conflict_tests/mod.rs"]
mod rs_fmt_08_dual_file_conflict_tests;
