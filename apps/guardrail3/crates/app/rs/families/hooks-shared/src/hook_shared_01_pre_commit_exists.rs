use guardrail3_domain_report::{CheckResult, Severity};

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

#[cfg(test)]
pub(crate) fn run_case(pre_commit_content: Option<&str>) -> Vec<CheckResult> {
    let pre_commit = pre_commit_content.map(|content| HookScriptFacts {
        rel_path: ".githooks/pre-commit".to_owned(),
        kind: super::facts::HookScriptKind::PreCommit,
        content: content.to_owned(),
    });
    let mut results = Vec::new();
    check(pre_commit.as_ref(), &mut results);
    results
}

#[cfg(test)]
#[path = "hook_shared_01_pre_commit_exists_tests/mod.rs"]
mod hook_shared_01_pre_commit_exists_tests;
