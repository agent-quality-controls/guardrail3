use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DispatcherSyntaxInput;

const ID: &str = "HOOK-SHARED-04";

pub fn check(input: &DispatcherSyntaxInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.has_modular_dir {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "monolithic hook mode".to_owned(),
                "No modular `pre-commit.d` directory exists, so no dispatcher is required."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    let has_dispatcher = input
        .parsed
        .executable_lines()
        .iter()
        .any(|line| line.is_dispatcher_syntax() && targets_pre_commit_dir(line.raw()));

    if has_dispatcher {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "dispatcher pattern present".to_owned(),
                "Modular hook layout is backed by a real dispatcher command.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "dispatcher pattern missing".to_owned(),
            "Modular hook layout exists but the dispatcher does not execute pre-commit.d."
                .to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
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

mod tests;
