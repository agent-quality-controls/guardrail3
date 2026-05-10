use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;

use super::support::{collect_precommit_scope_values, unquote_scope};

/// `ID` constant.
const ID: &str = "g3rs-hooks/routing-no-upward-walk-from-units";

/// `ANCESTOR_SCOPE_NAMES` constant.
const ANCESTOR_SCOPE_NAMES: &[&str] = &[
    "parent",
    "ancestor",
    "parent_unit",
    "ancestor_unit",
    "parent_dir",
    "ancestor_dir",
    "owning_parent",
];

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let scopes = collect_precommit_scope_values(input.parsed);
    let mut violation = None;

    for scope in &scopes {
        let body = unquote_scope(scope);
        if scope_value_is_ancestor_walk(body) {
            violation = Some(format!(
                "verifier scope `{scope}` is derived from an ancestor walk over previously discovered units"
            ));
            break;
        }
    }

    if violation.is_none()
        && let Some(detail) = ancestor_walk_assignment(input.parsed)
    {
        violation = Some(detail);
    }

    if let Some(detail) = violation {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "ancestor expansion of discovered units".to_owned(),
            format!(
                "{detail}. Routing must perform one upward walk per staged file to the nearest \
                 owning adopted unit, dedup, and stop. Calling the verifier on ancestor adopted \
                 units is forbidden."
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
            "no ancestor expansion of discovered units".to_owned(),
            ".githooks/pre-commit does not call the Rust verifier on ancestor adopted units."
                .to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory(),
    );
}

/// `scope_value_is_ancestor_walk` function.
fn scope_value_is_ancestor_walk(body: &str) -> bool {
    if body.contains("$(dirname") {
        return true;
    }
    let candidate = body.trim_start_matches('"').trim_end_matches('"');
    let candidate = candidate.trim_start_matches('$').trim_start_matches('{');
    let candidate = candidate.trim_end_matches('}');
    ANCESTOR_SCOPE_NAMES.contains(&candidate)
}

/// `ancestor_walk_assignment` function.
fn ancestor_walk_assignment(
    parsed: &hook_shell_parser::types::ParsedShellScript,
) -> Option<String> {
    for line in &parsed.source_lines {
        let trimmed = line.raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if !trimmed.contains("dirname") {
            continue;
        }
        // Look for `parent=...dirname "$unit"...` style lines.
        for name in ANCESTOR_SCOPE_NAMES {
            let prefix = format!("{name}=");
            if trimmed.contains(prefix.as_str()) {
                return Some(format!(
                    "line {} assigns `{name}` from a `dirname` invocation, indicating an ancestor walk over discovered units",
                    line.line_no
                ));
            }
        }
    }
    None
}
