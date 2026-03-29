use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DispatcherSyntaxInput;

const ID: &str = "HOOK-SHARED-19";

pub fn check(input: &DispatcherSyntaxInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.has_modular_dir {
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
                severity: Severity::Info,
                title: "dispatcher uses real executable syntax".to_owned(),
                message: "pre-commit.d/ is dispatched by a real executable shell command."
                    .to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "dispatcher syntax missing".to_owned(),
            message:
                "pre-commit.d/ exists but no executable dispatcher command sources or runs it."
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
#[path = "hook_shared_19_real_dispatcher_syntax_only_tests/mod.rs"]
mod hook_shared_19_real_dispatcher_syntax_only_tests;
