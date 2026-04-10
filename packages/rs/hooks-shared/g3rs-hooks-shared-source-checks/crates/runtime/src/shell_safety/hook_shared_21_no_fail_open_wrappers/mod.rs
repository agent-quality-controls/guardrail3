use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::FailOpenWrapperInput;

const ID: &str = "HOOK-SHARED-21";

pub(crate) fn check(input: &FailOpenWrapperInput<'_>, results: &mut Vec<G3CheckResult>) {
    for line in input.executable_lines {
        if line.softened_by().is_none() || !is_guardrail_critical(line.command_text()) {
            continue;
        }

        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "critical hook command is fail-open".to_owned(),
            format!(
                "Critical hook command `{}` is softened by a fail-open wrapper.",
                line.command_text()
            ),
            Some(input.rel_path.to_owned()),
            Some(line.line_no()),
            false,
        ));
    }
}

fn is_guardrail_critical(command_text: &str) -> bool {
    let command_name = command_text.split_whitespace().next().unwrap_or_default();
    command_name == "guardrail3"
        || command_name == "gitleaks"
        || command_name == "cargo-deny"
        || command_name == "cargo-machete"
        || command_name == "cargo-dupes"
        || (command_name == "cargo"
            && (command_text.contains("cargo clippy")
                || command_text.contains("cargo deny")
                || command_text.contains("cargo test")
                || command_text.contains("cargo machete")
                || command_text.contains("cargo dupes")))
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        executable_lines: parsed.executable_lines(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
