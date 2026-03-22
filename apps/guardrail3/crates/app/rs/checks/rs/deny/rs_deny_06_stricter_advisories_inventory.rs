use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{expected_advisory_baseline, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(advisories) = section(config, "advisories") else {
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
    if matches!(actual, Some("deny")) && expected != "deny" {
        results.push(
            CheckResult {
                id: "RS-DENY-06".to_owned(),
                severity: Severity::Info,
                title: format!("advisories `{key}` stricter than baseline"),
                message: format!("`{}` sets `[advisories].{key} = \"deny\"`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_deny_06_stricter_advisories_inventory_tests.rs"]
mod tests;
