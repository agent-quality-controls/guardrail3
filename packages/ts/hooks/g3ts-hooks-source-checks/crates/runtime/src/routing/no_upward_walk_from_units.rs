use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use super::support::{collect_precommit_scope_values, unquote_scope};

/// Result identifier for the no-upward-walk routing rule.
const ID: &str = "g3ts-hooks/routing-no-upward-walk-from-units";

/// Variable names that flag an ancestor walk over discovered owning units.
const ANCESTOR_SCOPE_NAMES: &[&str] = &[
    "parent",
    "ancestor",
    "parent_unit",
    "ancestor_unit",
    "parent_dir",
    "ancestor_dir",
    "owning_parent",
];

/// Records a finding when the hook walks ancestors of discovered owning units.
pub(crate) fn check(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let scopes = collect_precommit_scope_values(parsed);
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

    if violation.is_none() {
        if let Some(detail) = ancestor_walk_assignment(parsed) {
            violation = Some(detail);
        }
    }

    if let Some(detail) = violation {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "ancestor expansion of discovered units".to_owned(),
            format!(
                "{detail}. Routing must perform one upward walk per staged file to the nearest \
                 owning adopted unit, dedup, and stop. Calling the verifier on ancestor adopted \
                 units is forbidden."
            ),
            Some(input.rel_path().to_owned()),
            None,
        ));
    }
}

/// Returns true when `body` resolves to an ancestor-walk scope (e.g. `$(dirname ...)`).
fn scope_value_is_ancestor_walk(body: &str) -> bool {
    if body.contains("$(dirname") {
        return true;
    }
    let candidate = body.trim_start_matches('"').trim_end_matches('"');
    let candidate = candidate.trim_start_matches('$').trim_start_matches('{');
    let candidate = candidate.trim_end_matches('}');
    ANCESTOR_SCOPE_NAMES.contains(&candidate)
}

/// Returns a description of any line that assigns an ancestor-name variable from `dirname`.
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
