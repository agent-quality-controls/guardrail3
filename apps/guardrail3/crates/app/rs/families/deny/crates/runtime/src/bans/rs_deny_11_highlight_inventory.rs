use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_bans_settings, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(bans) = section(config, "bans") else {
        return;
    };
    let (_, _, expected_highlight) = expected_bans_settings();
    let actual = bans.get("highlight").and_then(toml::Value::as_str);
    if actual.map(str::to_owned) != expected_highlight {
        results.push(
            CheckResult::from_parts(
                "RS-DENY-11".to_owned(),
                Severity::Info,
                "highlight differs from baseline".to_owned(),
                format!(
                    "`{}` sets `[bans].highlight = {}`.",
                    config.rel_path,
                    actual.unwrap_or("<missing>")
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
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
pub(crate) use ::test_support::{build_fixture_deny_toml, remove_section_key, set_section_string};
#[cfg(test)]
#[path = "rs_deny_11_highlight_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_11_highlight_inventory_tests;
