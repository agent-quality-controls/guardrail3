use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_licenses, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        results.push(CheckResult {
            id: "RS-DENY-14".to_owned(),
            severity: Severity::Error,
            title: "[licenses] section missing".to_owned(),
            message: format!("`{}` has no `[licenses]` section.", config.rel_path),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };

    let actual: std::collections::BTreeSet<String> = licenses
        .get("allow")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    for name in expected_licenses() {
        if !actual.contains(&name) {
            results.push(CheckResult {
                id: "RS-DENY-14".to_owned(),
                severity: Severity::Error,
                title: "baseline license missing".to_owned(),
                message: format!("`{}` is missing allowed license `{name}`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    let private_ignore = licenses
        .get("private")
        .and_then(|value| value.get("ignore"))
        .and_then(toml::Value::as_bool);
    if private_ignore != Some(true) {
        results.push(CheckResult {
            id: "RS-DENY-14".to_owned(),
            severity: Severity::Error,
            title: "licenses.private.ignore must be true".to_owned(),
            message: format!(
                "`{}` must set `[licenses.private].ignore = true`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, remove_allowed_license, remove_section, set_private_ignore,
};
#[cfg(test)]
pub(crate) fn expected_licenses_for_test() -> std::collections::BTreeSet<String> {
    super::deny_support::expected_licenses()
}
#[cfg(test)]
#[cfg(test)]
#[path = "rs_deny_14_license_allow_baseline_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_14_license_allow_baseline_tests;
