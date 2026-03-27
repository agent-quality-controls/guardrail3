use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtDualConflictInput;

const ID: &str = "RS-FMT-08";

pub fn check(input: &RustfmtDualConflictInput, results: &mut Vec<CheckResult>) {
    let file = if input.dir_rel.is_empty() {
        "rustfmt.toml".to_owned()
    } else {
        ProjectTree::join_rel(&input.dir_rel, "rustfmt.toml")
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "Conflicting rustfmt config files".to_owned(),
        message: "Both rustfmt.toml and .rustfmt.toml exist in the same directory".to_owned(),
        file: Some(file),
        line: None,
        inventory: false,
    });
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
#[path = "rs_fmt_08_dual_file_conflict_tests/mod.rs"]
mod rs_fmt_08_dual_file_conflict_tests;
