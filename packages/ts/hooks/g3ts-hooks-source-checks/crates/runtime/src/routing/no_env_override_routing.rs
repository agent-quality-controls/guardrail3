use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use super::support::{
    contains_command_substitution_default, contains_default_fallback_assignment_for,
    contains_env_default_substitution, precommit_scope_feeder_variable_names,
};

/// Result identifier for the no-env-override routing rule.
const ID: &str = "g3ts-hooks/routing-no-env-override";

/// Environment variable names that must not influence verifier scope.
const FORBIDDEN_ENV_VARS: &[&str] = &[
    "GUARDRAIL3_TS_PACKAGE",
    "GUARDRAIL3_TYPESCRIPT_PACKAGE",
    "GUARDRAIL3_TS_SCOPE",
    "GUARDRAIL3_TYPESCRIPT_SCOPE",
];

/// Records a finding when the hook permits env-driven scope routing.
pub(crate) fn check(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let mut findings = Vec::new();

    let scope_var_names = precommit_scope_feeder_variable_names(parsed);

    for line in &parsed.source_lines {
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
        if contains_command_substitution_default(line.raw.as_str())
            && let Some(lhs) = extract_assignment_lhs(trimmed)
            && scope_var_names.iter().any(|name| name == &lhs)
        {
            findings.push(format!(
                "`.githooks/pre-commit` line {} assigns `{lhs}` from a command-substitution default (`$(... || ...)`) and later passes `${lhs}` as the verifier `--path`. Routing must not rely on default scopes.",
                line.line_no
            ));
        }
    }

    if let Some(name) = contains_default_fallback_assignment_for(parsed, &scope_var_names) {
        findings.push(format!(
            "`.githooks/pre-commit` contains a default-fallback assignment to `{name}` (`if [ -z \"${name}\" ]; then {name}=<literal>; fi`) that is later passed as the verifier `--path`. Routing must not rely on default scopes."
        ));
    }

    if findings.is_empty() {
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
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "env-override routing in pre-commit hook".to_owned(),
        message,
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Returns the left-hand side identifier of a shell assignment (`name=value`), if valid.
fn extract_assignment_lhs(trimmed: &str) -> Option<String> {
    let line = trimmed
        .strip_prefix("export ")
        .unwrap_or(trimmed)
        .trim_start();
    let (lhs, _rhs) = line.split_once('=')?;
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
