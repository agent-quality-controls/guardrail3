use crate::domain::report::{CheckResult, Severity};

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
        .any(|line| line.is_dispatcher_syntax && line.raw.contains("pre-commit.d"));

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

#[cfg(test)]
#[path = "hook_shared_19_real_dispatcher_syntax_only_tests/mod.rs"]
mod tests;
