use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustfmtDualConflictInput;

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

