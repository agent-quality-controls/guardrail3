#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::too_many_lines,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;

use super::support::{
    contains_command_substitution_default, contains_default_fallback_assignment_for,
    contains_env_default_substitution, precommit_scope_feeder_variable_names,
};

/// `ID` constant.
const ID: &str = "g3rs-hooks/routing-no-env-override";

/// `FORBIDDEN_ENV_VARS` constant.
const FORBIDDEN_ENV_VARS: &[&str] = &[
    "GUARDRAIL3_RUST_WORKSPACE",
    "GUARDRAIL3_RS_WORKSPACE",
    "GUARDRAIL3_RUST_SCOPE",
    "GUARDRAIL3_RS_SCOPE",
];

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut findings = Vec::new();

    let scope_var_names = precommit_scope_feeder_variable_names(input.parsed);

    for line in &input.parsed.source_lines {
        let trimmed = line.raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }

        for var in FORBIDDEN_ENV_VARS {
            if line.raw.contains(var) {
                findings.push(format!(
                    "`.githooks/pre-commit` references `{var}`. Environment-override routing is not allowed; routing must walk upward from each staged file to the owning adopted unit."
                ));
            }
        }
        if contains_env_default_substitution(line.raw.as_str()) {
            findings.push(format!(
                "`.githooks/pre-commit` line {} uses an env-default substitution that can produce the verifier scope. Routing must not rely on environment defaults.",
                line.line_no
            ));
        }
        if !contains_command_substitution_default(line.raw.as_str()) {
            continue;
        }
        let Some(lhs) = extract_assignment_lhs(trimmed) else {
            continue;
        };
        if scope_var_names.iter().any(|name| name == &lhs) {
            findings.push(format!(
                "`.githooks/pre-commit` line {} assigns `{lhs}` from a command-substitution default (`$(... || ...)`) and later passes `${lhs}` as the verifier `--scope`. Routing must not rely on default scopes.",
                line.line_no
            ));
        }
    }

    if let Some(name) = contains_default_fallback_assignment_for(input.parsed, &scope_var_names) {
        findings.push(format!(
            "`.githooks/pre-commit` contains a default-fallback assignment to `{name}` (`if [ -z \"${name}\" ]; then {name}=<literal>; fi`) that is later passed as the verifier `--scope`. Routing must not rely on default scopes."
        ));
    }

    if findings.is_empty() {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "no env-override routing".to_owned(),
                ".githooks/pre-commit does not use environment-override routing.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    let mut message = findings.join(" ");
    let summary_token = if findings
        .iter()
        .any(|f| f.contains("command-substitution default"))
    {
        "command-substitution default"
    } else if findings
        .iter()
        .any(|f| f.contains("default-fallback assignment"))
    {
        "default-fallback assignment"
    } else if findings.iter().any(|f| f.contains("env-default")) {
        "env-default substitution"
    } else {
        "Environment-override routing is not allowed"
    };
    if !message.contains(summary_token) {
        message.push(' ');
        message.push_str(summary_token);
    }
    results.push(G3CheckResult::from_parts(
        ID.to_owned(),
        G3Severity::Error,
        "env-override routing in pre-commit hook".to_owned(),
        message,
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `extract_assignment_lhs` function.
fn extract_assignment_lhs(trimmed: &str) -> Option<String> {
    // Strip a leading `export ` if present.
    let line = trimmed
        .strip_prefix("export ")
        .unwrap_or(trimmed)
        .trim_start();
    let eq_idx = line.find('=')?;
    let lhs = &line[..eq_idx];
    if lhs.is_empty() {
        return None;
    }
    if lhs
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        && !lhs.chars().next().is_some_and(|ch| ch.is_ascii_digit())
    {
        return Some(lhs.to_owned());
    }
    None
}
