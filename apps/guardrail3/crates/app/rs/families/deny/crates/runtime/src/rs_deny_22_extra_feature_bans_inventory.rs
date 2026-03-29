use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::parse_feature_entries_in_config;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    for entry in parse_feature_entries_in_config(parsed) {
        if entry.name != "tokio" {
            results.push(
                CheckResult {
                    id: "RS-DENY-22".to_owned(),
                    severity: Severity::Info,
                    title: "extra feature ban".to_owned(),
                    message: format!(
                        "`{}` has extra feature-ban entry for `{}`.",
                        config.rel_path, entry.name
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
    build_fixture_deny_toml, copy_fixture, set_feature_entries, write_file,
};
#[cfg(test)]
pub(crate) fn parse_feature_entries_for_test(
    parsed: &toml::Value,
) -> Vec<super::deny_support::FeatureConfigEntry> {
    super::deny_support::parse_feature_entries_in_config(parsed)
}
#[cfg(test)]
#[path = "rs_deny_22_extra_feature_bans_inventory_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_deny_22_extra_feature_bans_inventory_tests;
