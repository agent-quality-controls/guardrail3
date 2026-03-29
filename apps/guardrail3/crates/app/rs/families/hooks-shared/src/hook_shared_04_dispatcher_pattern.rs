use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DispatcherSyntaxInput;

const ID: &str = "HOOK-SHARED-04";

pub fn check(input: &DispatcherSyntaxInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.has_modular_dir {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "monolithic hook mode".to_owned(),
                message:
                    "No modular `pre-commit.d` directory exists, so no dispatcher is required."
                        .to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    let has_dispatcher = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line.is_dispatcher_syntax && targets_pre_commit_dir(line.raw));

    if has_dispatcher {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "dispatcher pattern present".to_owned(),
                message: "Modular hook layout is backed by a real dispatcher command.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "dispatcher pattern missing".to_owned(),
            message: "Modular hook layout exists but the dispatcher does not execute pre-commit.d."
                .to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn targets_pre_commit_dir(raw: &str) -> bool {
    let normalized = raw.replace(['"', '\''], "");
    normalized.contains("pre-commit.d/") || normalized.ends_with("pre-commit.d")
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_case(content: &str, has_modular_dir: bool) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_shared_04_dispatcher_pattern_tests/mod.rs"]
mod hook_shared_04_dispatcher_pattern_tests;
