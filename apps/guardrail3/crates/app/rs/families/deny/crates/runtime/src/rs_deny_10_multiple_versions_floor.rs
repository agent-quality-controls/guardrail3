use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        results.push(CheckResult {
            id: "RS-DENY-10".to_owned(),
            severity: Severity::Warn,
            title: "[bans] section missing".to_owned(),
            message: format!("`{}` has no `[bans]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    match bans.get("multiple-versions").and_then(toml::Value::as_str) {
        Some("deny") => {}
        Some(other) => results.push(CheckResult {
            id: "RS-DENY-10".to_owned(),
            severity: Severity::Warn,
            title: "multiple-versions weaker than baseline".to_owned(),
            message: format!(
                "`{}` sets `[bans].multiple-versions = \"{other}\"`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: "RS-DENY-10".to_owned(),
            severity: Severity::Warn,
            title: "multiple-versions missing".to_owned(),
            message: format!(
                "`{}` does not set `[bans].multiple-versions`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, remove_section, remove_section_key, set_section_string,
    write_file,
};
#[cfg(test)]
#[path = "rs_deny_10_multiple_versions_floor_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_10_multiple_versions_floor_tests;
