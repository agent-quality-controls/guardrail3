use crate::domain::report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-02";

pub fn check(hooks_path: Option<&str>, results: &mut Vec<CheckResult>) {
    match hooks_path {
        Some(".githooks") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "core.hooksPath configured".to_owned(),
                message: "git config core.hooksPath points to `.githooks`.".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(actual) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "core.hooksPath has wrong value".to_owned(),
            message: format!("Expected `.githooks`, got `{actual}`."),
            file: None,
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "core.hooksPath not configured".to_owned(),
            message: "git config core.hooksPath does not resolve to `.githooks`.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }),
    }
}
