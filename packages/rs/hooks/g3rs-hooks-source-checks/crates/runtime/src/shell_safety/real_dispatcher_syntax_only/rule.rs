use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::DispatcherSyntaxInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/real-dispatcher-syntax-only";

/// `check` function.
pub(crate) fn check(input: &DispatcherSyntaxInput<'_>, results: &mut Vec<G3CheckResult>) {
    if !input.has_modular_dir {
        return;
    }

    let has_dispatcher = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line.is_dispatcher_syntax && targets_pre_commit_dir(&line.raw));

    if has_dispatcher {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "dispatcher uses real executable syntax".to_owned(),
                "pre-commit.d/ is dispatched by a real executable shell command.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "dispatcher syntax missing".to_owned(),
            "pre-commit.d/ exists but no executable dispatcher command sources or runs it."
                .to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

/// `targets_pre_commit_dir` function.
fn targets_pre_commit_dir(raw: &str) -> bool {
    let normalized = raw.replace(['"', '\''], "");
    normalized.contains("pre-commit.d/") || normalized.ends_with("pre-commit.d")
}

#[cfg(test)]
pub(crate) fn run_case(
    content: &str,
    has_modular_dir: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
