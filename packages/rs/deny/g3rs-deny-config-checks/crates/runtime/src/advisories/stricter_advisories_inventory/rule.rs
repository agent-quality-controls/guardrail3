use deny_toml_parser::types::{AdvisoryScope, DenyToml};
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_advisory_baseline;
use crate::support::findings::inventory;

const ID: &str = "g3rs-deny/stricter-advisories-inventory";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };
    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();

    check_value(
        deny_rel_path,
        advisories.unmaintained.as_ref().map(advisory_scope_str),
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

const fn advisory_scope_str(scope: &AdvisoryScope) -> &'static str {
    match scope {
        AdvisoryScope::All => "all",
        AdvisoryScope::Workspace => "workspace",
        AdvisoryScope::Transitive => "transitive",
        AdvisoryScope::None => "none",
    }
}

fn check_value(
    deny_rel_path: &str,
    actual: Option<&str>,
    key: &str,
    expected: &str,
    results: &mut Vec<G3CheckResult>,
) {
    if is_stricter(actual, expected) {
        results.push(inventory(
            ID,
            format!("advisories `{key}` stricter than baseline"),
            format!(
                "`{deny_rel_path}` sets `[advisories].{key} = \"{}\"`.",
                actual.unwrap_or_default()
            ),
            deny_rel_path,
        ));
    }
}

fn is_stricter(actual: Option<&str>, expected: &str) -> bool {
    match (actual, advisory_rank(expected)) {
        (Some(actual), Some(expected_rank)) => {
            advisory_rank(actual).is_some_and(|actual_rank| actual_rank > expected_rank)
        }
        _ => false,
    }
}

fn advisory_rank(value: &str) -> Option<u8> {
    match value {
        "none" => Some(0),
        "transitive" => Some(1),
        "workspace" => Some(2),
        "all" => Some(3),
        "allow" => Some(0),
        "warn" => Some(1),
        "deny" => Some(2),
        _ => None,
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
