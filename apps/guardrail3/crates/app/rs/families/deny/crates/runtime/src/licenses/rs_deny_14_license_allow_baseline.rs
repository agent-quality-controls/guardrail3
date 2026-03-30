use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_licenses, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "[licenses] section missing".to_owned(),
            format!("`{}` has no `[licenses]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    let expected = expected_licenses();
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

    for name in &expected {
        if !actual.contains(name.as_str()) {
            results.push(CheckResult::from_parts(
                "RS-DENY-14".to_owned(),
                Severity::Error,
                "baseline license missing".to_owned(),
                format!("`{}` is missing allowed license `{name}`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    for name in actual.difference(&expected) {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "unexpected allowed license".to_owned(),
            format!("`{}` allows unexpected license `{name}`.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }

    let private_ignore = licenses
        .get("private")
        .and_then(|value| value.get("ignore"))
        .and_then(toml::Value::as_bool);
    if private_ignore != Some(true) {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "licenses.private.ignore must be true".to_owned(),
            format!(
                "`{}` must set `[licenses.private].ignore = true`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_allowed_license, build_fixture_deny_toml, remove_allowed_license, remove_section,
    set_private_ignore,
};
#[cfg(test)]
pub(crate) fn expected_licenses_for_test() -> std::collections::BTreeSet<String> {
    super::deny_support::expected_licenses()
}
#[cfg(test)]
#[path = "rs_deny_14_license_allow_baseline_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_deny_14_license_allow_baseline_tests;
