use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use super::support::{collect_precommit_scope_values, scope_is_wholly_variable, unquote_scope};

/// Result identifier for the hardcoded-scope-literal routing rule.
const ID: &str = "g3ts-hooks/routing-scope-not-hardcoded-literal";

/// Ambient shell variable names that may not stand in for a per-unit scope.
const DISALLOWED_AMBIENT_SCOPE_VARS: &[&str] = &["REPO_ROOT", "PWD", "HOME", "SCOPE"];

/// Records a finding when the hook calls the verifier with a hardcoded or ambient scope.
pub(crate) fn check(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let scopes = collect_precommit_scope_values(input.parsed());
    if scopes.is_empty() {
        return;
    }

    if let Some(value) = scopes
        .iter()
        .find(|value| scope_is_disallowed_ambient_variable(value))
    {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "verifier scope is a disallowed ambient variable".to_owned(),
            format!(
                "`.githooks/pre-commit` calls the TS verifier with `--path {value}`, an \
                 ambient shell variable that is not the per-staged-file discovery loop iteration \
                 variable. Routing must walk upward from each staged file to its owning adopted \
                 unit and pass that unit path as a loop variable, not an ambient `$REPO_ROOT`, \
                 `$PWD`, `$HOME`, or `$SCOPE`."
            ),
            Some(input.rel_path().to_owned()),
            None,
        ));
        return;
    }

    if let Some(value) = scopes
        .iter()
        .find(|value| scope_is_hardcoded_unit_path(value))
    {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "verifier scope is a hardcoded unit path".to_owned(),
            format!(
                "`.githooks/pre-commit` calls the TS verifier with a hardcoded scope `{value}`. \
                 Routing must walk upward from each staged file to its owning adopted unit and pass \
                 that unit path as a loop variable, not a literal."
            ),
            Some(input.rel_path().to_owned()),
            None,
        ));
    }
}

/// Returns true when `value` is `$VAR`/`${VAR}` for an ambient shell variable name.
fn scope_is_disallowed_ambient_variable(value: &str) -> bool {
    let body = unquote_scope(value).trim();
    let name = if let Some(rest) = body.strip_prefix("${") {
        if let Some(name) = rest.strip_suffix('}') {
            name
        } else {
            return false;
        }
    } else if let Some(name) = body.strip_prefix('$') {
        name
    } else {
        return false;
    };
    DISALLOWED_AMBIENT_SCOPE_VARS.contains(&name)
}

/// Returns true when `value` contains a hardcoded literal path segment after expansions are masked.
fn scope_is_hardcoded_unit_path(value: &str) -> bool {
    if scope_is_wholly_variable(value) {
        return false;
    }
    let body = unquote_scope(value);
    contains_literal_path_segment(body)
}

/// Returns true when `value` contains a literal path segment.
///
/// Mirrors the original byte-walk semantics:
/// - if a `/` is followed (anywhere later in the raw string) by an alnum/`.`/`-` byte, returns true;
/// - or if the value begins with a run of alnum/`.`/`-` followed immediately by `/`, returns true.
fn contains_literal_path_segment(value: &str) -> bool {
    if any_slash_followed_by_literal(value) {
        return true;
    }
    starts_with_literal_then_slash(value)
}

/// Returns true if some `/` byte in `value` is followed (anywhere later) by an alnum/`.`/`-` byte.
fn any_slash_followed_by_literal(value: &str) -> bool {
    let mut iter = value.bytes();
    while let Some(byte) = iter.next() {
        if byte != b'/' {
            continue;
        }
        if iter.clone().any(is_literal_path_byte) {
            return true;
        }
    }
    false
}

/// Returns true if `value` begins with a run of literal-path bytes followed by `/`.
fn starts_with_literal_then_slash(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !is_literal_path_byte(first) {
        return false;
    }
    for byte in bytes {
        if is_literal_path_byte(byte) {
            continue;
        }
        return byte == b'/';
    }
    false
}

/// Returns true when `byte` is part of a literal path segment.
const fn is_literal_path_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'-'
}
