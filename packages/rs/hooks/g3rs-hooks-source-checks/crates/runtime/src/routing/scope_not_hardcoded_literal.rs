#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::needless_continue,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;

use super::support::{collect_precommit_scope_values, scope_is_wholly_variable, unquote_scope};

/// `ID` constant.
const ID: &str = "g3rs-hooks/routing-scope-not-hardcoded-literal";

/// Ambient shell variables that are never the per-staged-file discovery loop iteration
/// variable. Passing one of these as the verifier `--scope` argument means the hook is
/// not routing through the marker-pair discovery loop. `REPO_ROOT` in particular is an
/// ancestor-of-everything expansion that bypasses both this rule's prior literal-path
/// check and `routing-no-upward-walk-from-units`.
const DISALLOWED_AMBIENT_SCOPE_VARS: &[&str] = &["REPO_ROOT", "PWD", "HOME", "SCOPE"];

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let scopes = collect_precommit_scope_values(input.parsed);
    if scopes.is_empty() {
        // reason: gated by dispatches-per-unit-validate-staged; nothing to evaluate here.
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "verifier scope not hardcoded".to_owned(),
                ".githooks/pre-commit does not invoke the Rust verifier with a hardcoded unit path."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    if let Some(value) = scopes
        .iter()
        .find(|value| scope_is_disallowed_ambient_variable(value))
    {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "verifier scope is a disallowed ambient variable".to_owned(),
            format!(
                "`.githooks/pre-commit` calls the Rust verifier with `--scope {value}`, an \
                 ambient shell variable that is not the per-staged-file discovery loop iteration \
                 variable. Routing must walk upward from each staged file to its owning adopted \
                 unit and pass that unit path as a loop variable, not an ambient `$REPO_ROOT`, \
                 `$PWD`, `$HOME`, or `$SCOPE`."
            ),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    let bad_scope = scopes
        .iter()
        .find(|value| scope_is_hardcoded_unit_path(value));
    if let Some(value) = bad_scope {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "verifier scope is a hardcoded unit path".to_owned(),
            format!(
                "`.githooks/pre-commit` calls the Rust verifier with a hardcoded scope `{value}`. \
                 Routing must walk upward from each staged file to its owning adopted unit and pass \
                 that unit path as a loop variable, not a literal."
            ),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    results.push(
        G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "verifier scope not hardcoded".to_owned(),
            ".githooks/pre-commit invokes the Rust verifier with a discovery loop variable."
                .to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory(),
    );
}

/// `scope_is_disallowed_ambient_variable` function.
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

/// `scope_is_hardcoded_unit_path` function.
fn scope_is_hardcoded_unit_path(value: &str) -> bool {
    if scope_is_wholly_variable(value) {
        return false;
    }
    let body = unquote_scope(value);
    // Treat any scope value that contains a literal path segment (a slash followed by literal text)
    // as a hardcoded path. Loop-variable scopes pass through `scope_is_wholly_variable` above.
    contains_literal_path_segment(body)
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// `contains_literal_path_segment` function.
fn contains_literal_path_segment(value: &str) -> bool {
    let mut i = 0;
    let bytes = value.as_bytes();
    let mut found_literal_segment = false;
    while i < bytes.len() {
        match bytes[i] {
            b'$' => {
                if bytes.get(i + 1) == Some(&b'{') {
                    let mut depth = 1;
                    let mut j = i + 2;
                    while j < bytes.len() && depth > 0 {
                        match bytes[j] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => {}
                        }
                        if depth > 0 {
                            j += 1;
                        }
                    }
                    i = j + 1;
                    continue;
                }
                // bare $NAME
                let mut j = i + 1;
                while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                i = j;
                continue;
            }
            b'/' => {
                // A bare slash with literal text after counts as a literal path segment.
                let after = &value[i + 1..];
                if after
                    .bytes()
                    .any(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
                {
                    found_literal_segment = true;
                }
                i += 1;
            }
            ch if ch.is_ascii_alphanumeric() || ch == b'.' || ch == b'-' => {
                // Literal text outside variable expansion. Consume until next special char.
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'-')
                {
                    i += 1;
                }
                if start == 0 {
                    // Looks like a literal path or path component starting at the beginning, e.g. `apps/...`.
                    // Continue and let the slash branch confirm we have a path-like literal.
                    found_literal_segment = i < bytes.len() && bytes[i] == b'/';
                } else {
                    // Literal text after a variable expansion, e.g. `$REPO_ROOT/apps/...` -> trips the slash branch.
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    found_literal_segment
}
