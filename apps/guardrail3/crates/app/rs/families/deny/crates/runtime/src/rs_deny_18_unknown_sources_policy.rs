use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_sources, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        results.push(CheckResult {
            id: "RS-DENY-18".to_owned(),
            severity: Severity::Error,
            title: "[sources] section missing".to_owned(),
            message: format!("`{}` has no `[sources]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };
    let (_, expected_unknown_registry, expected_unknown_git) = expected_sources();
    for (key, expected) in [
        ("unknown-registry", expected_unknown_registry),
        ("unknown-git", expected_unknown_git),
    ] {
        match sources.get(key).and_then(toml::Value::as_str) {
            Some(value) if value == expected => {}
            _ => results.push(CheckResult {
                id: "RS-DENY-18".to_owned(),
                severity: Severity::Error,
                title: format!("sources `{key}` has wrong value"),
                message: format!(
                    "`{}` must set `[sources].{key} = \"{expected}\"`.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }),
        }
    }
}


#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_check_with_profile(deny_toml: &str, profile_name: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, Some(profile_name), check)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, remove_section, set_source_policy};
#[cfg(test)]
pub(crate) fn expected_sources_for_test() -> (
    std::collections::BTreeSet<String>,
    String,
    String,
) {
    super::deny_support::expected_sources()
}
#[cfg(test)]
#[path = "rs_deny_18_unknown_sources_policy_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_18_unknown_sources_policy_tests;
