use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    if let Some(exceptions) = licenses.get("exceptions").and_then(toml::Value::as_array) {
        for entry in exceptions {
            if let Some(name) = entry
                .get("name")
                .or_else(|| entry.get("crate"))
                .and_then(toml::Value::as_str)
            {
                results.push(
                    CheckResult {
                        id: "RS-DENY-17".to_owned(),
                        severity: Severity::Info,
                        title: "license exception entry".to_owned(),
                        message: format!(
                            "`{}` has license exception for `{name}`.",
                            config.rel_path
                        ),
                        file: Some(config.rel_path.clone()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
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
pub(crate) use ::test_support::{build_fixture_deny_toml, copy_fixture, set_license_exceptions, write_file};
#[cfg(test)]
#[path = "rs_deny_17_license_exceptions_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_17_license_exceptions_inventory_tests;
