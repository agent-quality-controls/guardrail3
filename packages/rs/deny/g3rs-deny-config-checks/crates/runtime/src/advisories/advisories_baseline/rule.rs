use deny_toml_parser::types::{AdvisoryScope, DenyToml};
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_advisory_baseline;
use crate::support::findings::error;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/advisories-baseline";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        results.push(error(
            ID,
            "[advisories] section missing",
            format!("`{deny_rel_path}` has no `[advisories]` section."),
            deny_rel_path,
        ));
        return;
    };

    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();
    check_value(
        deny_rel_path,
        advisories.unmaintained.map(advisory_scope_str),
        "unmaintained",
        &expected_unmaintained,
        results,
    );
    check_value(
        deny_rel_path,
        advisories.yanked.as_deref(),
        "yanked",
        &expected_yanked,
        results,
    );
}

/// Implements `advisory scope str`.
const fn advisory_scope_str(scope: AdvisoryScope) -> &'static str {
    match scope {
        AdvisoryScope::All => "all",
        AdvisoryScope::Workspace => "workspace",
        AdvisoryScope::Transitive => "transitive",
        AdvisoryScope::None => "none",
    }
}

/// Implements `check value`.
fn check_value(
    deny_rel_path: &str,
    actual: Option<&str>,
    key: &str,
    expected: &str,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(value) if value == expected => {}
        Some(value) => results.push(error(
            ID,
            format!("advisories `{key}` has wrong value"),
            format!(
                "`{deny_rel_path}` must set `[advisories].{key} = \"{expected}\"`, found `{value}`."
            ),
            deny_rel_path,
        )),
        None => results.push(error(
            ID,
            format!("advisories `{key}` missing"),
            format!("`{deny_rel_path}` must set `[advisories].{key} = \"{expected}\"`."),
            deny_rel_path,
        )),
    }
}
