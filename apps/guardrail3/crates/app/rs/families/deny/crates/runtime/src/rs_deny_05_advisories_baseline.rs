use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_advisory_baseline, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
        results.push(CheckResult {
            id: "RS-DENY-05".to_owned(),
            severity: Severity::Error,
            title: "[advisories] section missing".to_owned(),
            message: format!("`{}` has no `[advisories]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();
    check_value(
        advisories.get("unmaintained").and_then(toml::Value::as_str),
        "unmaintained",
        &expected_unmaintained,
        config,
        results,
    );
    check_value(
        advisories.get("yanked").and_then(toml::Value::as_str),
        "yanked",
        &expected_yanked,
        config,
        results,
    );
}

fn check_value(
    actual: Option<&str>,
    key: &str,
    expected: &str,
    config: &super::facts::DenyConfigFacts,
    results: &mut Vec<CheckResult>,
) {
    match actual {
        Some(value) if value == expected => {}
        Some(value) => results.push(CheckResult {
            id: "RS-DENY-05".to_owned(),
            severity: Severity::Error,
            title: format!("advisories `{key}` has wrong value"),
            message: format!(
                "`{}` must set `[advisories].{key} = \"{expected}\"`, found `{value}`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: "RS-DENY-05".to_owned(),
            severity: Severity::Error,
            title: format!("advisories `{key}` missing"),
            message: format!(
                "`{}` must set `[advisories].{key} = \"{expected}\"`.",
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
#[path = "rs_deny_05_advisories_baseline_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_05_advisories_baseline_tests;
