use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtDualConflictInput;

const ID: &str = "RS-FMT-08";

pub fn check(input: &RustfmtDualConflictInput<'_>, results: &mut Vec<CheckResult>) {
    let file = if input.dir_rel.is_empty() {
        "rustfmt.toml".to_owned()
    } else {
        ProjectTree::join_rel(input.dir_rel, "rustfmt.toml")
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
#[path = "rs_fmt_08_dual_file_conflict_tests.rs"]
mod tests;
