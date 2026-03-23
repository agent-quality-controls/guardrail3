use crate::domain::report::{CheckResult, Severity};

use super::facts::HookScriptFacts;

const ID: &str = "HOOK-SHARED-01";

pub fn check(pre_commit: Option<&HookScriptFacts>, results: &mut Vec<CheckResult>) {
    match pre_commit {
        Some(script) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "pre-commit hook exists".to_owned(),
                message: "Found cached pre-commit hook.".to_owned(),
                file: Some(script.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "pre-commit hook missing".to_owned(),
            message: "Expected a cached `.githooks/pre-commit` or `hooks/pre-commit` hook."
                .to_owned(),
            file: Some(".githooks/pre-commit".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}
