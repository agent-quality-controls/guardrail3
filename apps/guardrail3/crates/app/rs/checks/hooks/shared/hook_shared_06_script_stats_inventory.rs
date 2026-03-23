use crate::domain::report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-06";

pub fn check(rel_path: &str, content: &str, results: &mut Vec<CheckResult>) {
    let line_count = content.lines().count();
    let byte_count = content.len();
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "pre-commit script stats".to_owned(),
            message: format!("{line_count} lines, {byte_count} bytes"),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}
