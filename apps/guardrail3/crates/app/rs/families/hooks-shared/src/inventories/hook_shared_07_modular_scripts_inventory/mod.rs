use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::HookScriptFacts;

const ID: &str = "HOOK-SHARED-07";

pub fn check(scripts: &[HookScriptFacts], results: &mut Vec<CheckResult>) {
    if scripts.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no modular hook scripts".to_owned(),
                "No cached files found in `.githooks/pre-commit.d`.".to_owned(),
                Some(".githooks/pre-commit.d".to_owned()),
                None,
                false,
            )
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
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "modular hook scripts inventory".to_owned(),
            names,
            Some(".githooks/pre-commit.d".to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn run_case(script_names: &[&str]) -> Vec<CheckResult> {
    let scripts = script_names
        .iter()
        .map(|name| HookScriptFacts {
            rel_path: format!(".githooks/pre-commit.d/{name}"),
            kind: crate::facts::HookScriptKind::Modular,
            content: String::new(),
        })
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    check(&scripts, &mut results);
    results
}

#[cfg(test)]

mod hook_shared_07_modular_scripts_inventory_tests;
