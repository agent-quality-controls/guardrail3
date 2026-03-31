use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_bans_settings, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-12".to_owned(),
            Severity::Error,
            "[bans] section missing".to_owned(),
            format!("`{}` has no `[bans]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    let (_, expected, _) = expected_bans_settings();
    match bans
        .get("allow-wildcard-paths")
        .and_then(toml::Value::as_bool)
    {
        Some(value) if value == expected => {}
        _ => results.push(CheckResult::from_parts(
            "RS-DENY-12".to_owned(),
            Severity::Error,
            "allow-wildcard-paths must be true".to_owned(),
            format!(
                "`{}` must set `[bans].allow-wildcard-paths = true`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use crate::config_facts;
#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, remove_section, remove_section_key, set_section_bool,
};
#[cfg(test)]
#[path = "rs_deny_12_allow_wildcard_paths_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_12_allow_wildcard_paths_tests;
