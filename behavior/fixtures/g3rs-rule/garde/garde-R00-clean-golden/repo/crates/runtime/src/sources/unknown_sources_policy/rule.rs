use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_sources;
use crate::support::findings::error;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/unknown-sources-policy";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(sources) = deny.sources.as_ref() else {
        results.push(error(
            ID,
            "[sources] section missing",
            format!("`{deny_rel_path}` has no `[sources]` section."),
            deny_rel_path,
        ));
        return;
    };

    let (_, expected_unknown_registry, expected_unknown_git) = expected_sources();
    for (key, actual, expected) in [
        (
            "unknown-registry",
            sources.unknown_registry.as_deref(),
            expected_unknown_registry.as_str(),
        ),
        (
            "unknown-git",
            sources.unknown_git.as_deref(),
            expected_unknown_git.as_str(),
        ),
    ] {
        match actual {
            Some(value) if value == expected => {}
            _ => results.push(error(
                ID,
                format!("sources `{key}` has wrong value"),
                format!("`{deny_rel_path}` must set `[sources].{key} = \"{expected}\"`."),
                deny_rel_path,
            )),
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
