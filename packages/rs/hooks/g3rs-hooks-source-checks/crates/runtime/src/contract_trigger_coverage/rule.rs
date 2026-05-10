#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use std::collections::BTreeSet;

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;
use g3rs_hooks_contract_types::G3HookTriggerPattern;

/// `ID` constant.
const ID: &str = "g3rs-hooks/contract-trigger-coverage";

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let exact_paths = exact_trigger_paths(input);
    if exact_paths.is_empty() {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "hook contract has no exact trigger paths to prove".to_owned(),
                "Family hook contracts only declare glob or extension trigger patterns here. Glob trigger routing is not yet provable; legacy hook rules remain active.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    let Some(pattern_text) = extract_rust_relevant_pattern(input.parsed) else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "hook does not declare a RUST_RELEVANT_PATTERN".to_owned(),
            "`.githooks/pre-commit` must declare a `RUST_RELEVANT_PATTERN='(...)'` regex used by the Rust discovery loop. Without it, contract trigger coverage cannot be proven.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    };

    let unmatched: Vec<String> = exact_paths
        .into_iter()
        .filter(|path| !pattern_covers(&pattern_text, path))
        .collect();

    if unmatched.is_empty() {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "hook contract trigger coverage proven".to_owned(),
                "Every family-declared exact trigger path appears in `.githooks/pre-commit`'s `RUST_RELEVANT_PATTERN`.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::from_parts(
        ID.to_owned(),
        G3Severity::Error,
        "hook contract trigger coverage missing".to_owned(),
        format!(
            "`.githooks/pre-commit`'s `RUST_RELEVANT_PATTERN` does not match these family-owned exact trigger paths: {}. Either add an alternative for each missing path to the pattern, or remove the trigger from the family hook contract.",
            unmatched.join(", ")
        ),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `exact_trigger_paths` function.
fn exact_trigger_paths(input: &RustHookCommandInput<'_>) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for requirement in input.requirements {
        for pattern in &requirement.trigger_patterns {
            if let G3HookTriggerPattern::ExactPath(path) = pattern {
                let _ = paths.insert(path.clone());
            }
        }
    }
    paths.into_iter().collect()
}

/// Returns the body of the `RUST_RELEVANT_PATTERN='...'` shell assignment
/// in the parsed hook script, or `None` if no such assignment exists.
fn extract_rust_relevant_pattern(
    parsed: &hook_shell_parser::types::ParsedShellScript,
) -> Option<String> {
    for line in &parsed.source_lines {
        let raw = line.raw.as_str();
        let trimmed = raw.trim_start();
        if let Some(rest) = trimmed.strip_prefix("RUST_RELEVANT_PATTERN=") {
            // Strip surrounding quotes (single or double).
            let stripped = rest.trim();
            if stripped.len() >= 2
                && ((stripped.starts_with('\'') && stripped.ends_with('\''))
                    || (stripped.starts_with('"') && stripped.ends_with('"')))
            {
                return Some(stripped[1..stripped.len() - 1].to_owned());
            }
            return Some(stripped.to_owned());
        }
    }
    None
}

/// Returns true if the regex `pattern` (the body of `RUST_RELEVANT_PATTERN`)
/// has an alternative anchored at end-of-string for `path`. This does not
/// invoke a regex engine; it tests whether the literal "<escaped path>$"
/// appears in the pattern text. This handles both top-level-only forms
/// (e.g. `Cargo\.toml$`) and sub-path-aware forms (e.g. `(^|/)Cargo\.toml$`).
fn pattern_covers(pattern: &str, path: &str) -> bool {
    pattern.contains(&format!("{}$", regex_escape(path)))
}

/// Escapes regex meta characters so the resulting string is a literal
/// pattern matching `path`. The same escapes the hook author uses in their
/// `RUST_RELEVANT_PATTERN` text. Only the metas that appear in our trigger
/// path strings are escaped (`.`, `+`, `*`, `?`, `(`, `)`, `[`, `]`, `{`,
/// `}`, `|`, `^`, `$`, `\`).
fn regex_escape(path: &str) -> String {
    let mut out = String::with_capacity(path.len() + 4);
    for ch in path.chars() {
        match ch {
            '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "API takes owned Vec to keep signature stable across the contract surface; downstream callers pass ownership"
)]
#[cfg(test)]
pub(crate) fn run_case(
    content: &str,
    requirements: Vec<g3rs_hooks_contract_types::G3HookRequirement>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
        requirements: &requirements,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
