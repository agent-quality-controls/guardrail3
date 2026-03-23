use crate::domain::report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-03";

pub fn check(has_modular_dir: bool, results: &mut Vec<CheckResult>) {
    let (title, message, file) = if has_modular_dir {
        (
            "pre-commit.d directory exists",
            "Hook uses modular pre-commit scripts.",
            Some(".githooks/pre-commit.d".to_owned()),
        )
    } else {
        (
            "pre-commit.d directory missing",
            "Hook currently uses a monolithic pre-commit script.",
            Some(".githooks".to_owned()),
        )
    };

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: title.to_owned(),
            message: message.to_owned(),
            file,
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}
