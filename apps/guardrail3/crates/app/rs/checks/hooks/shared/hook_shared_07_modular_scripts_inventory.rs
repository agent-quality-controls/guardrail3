use crate::domain::report::{CheckResult, Severity};

use super::facts::HookScriptFacts;

const ID: &str = "HOOK-SHARED-07";

pub fn check(scripts: &[HookScriptFacts], results: &mut Vec<CheckResult>) {
    if scripts.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "no modular hook scripts".to_owned(),
                message: "No cached files found in `.githooks/pre-commit.d`.".to_owned(),
                file: Some(".githooks/pre-commit.d".to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    let names = scripts
        .iter()
        .map(|script| script.rel_path.clone())
        .collect::<Vec<_>>()
        .join(", ");
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "modular hook scripts inventory".to_owned(),
            message: names,
            file: Some(".githooks/pre-commit.d".to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "hook_shared_07_modular_scripts_inventory_tests.rs"]
mod tests;
