use crate::domain::report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-08";

pub fn check(rel_path: &str, content: &str, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "pre-commit file size".to_owned(),
            message: format!("{} bytes", content.len()),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}
