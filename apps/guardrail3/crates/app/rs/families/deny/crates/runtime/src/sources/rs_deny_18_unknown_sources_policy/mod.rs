use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_sources, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-18".to_owned(),
            Severity::Error,
            "[sources] section missing".to_owned(),
            format!("`{}` has no `[sources]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let (_, expected_unknown_registry, expected_unknown_git) = expected_sources();
    for (key, expected) in [
        ("unknown-registry", expected_unknown_registry),
        ("unknown-git", expected_unknown_git),
    ] {
        match sources.get(key).and_then(toml::Value::as_str) {
            Some(value) if value == expected => {}
            _ => results.push(CheckResult::from_parts(
                "RS-DENY-18".to_owned(),
                Severity::Error,
                format!("sources `{key}` has wrong value"),
                format!(
                    "`{}` must set `[sources].{key} = \"{expected}\"`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            )),
        }
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, remove_section, set_source_policy};
#[cfg(test)]
pub(crate) fn expected_sources_for_test() -> (std::collections::BTreeSet<String>, String, String) {
    crate::deny_support::expected_sources()
}
#[cfg(test)]

// reason: test-only sidecar module wiring
mod tests;
