use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::section;
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        return;
    };
    if let Some(allow_git) = sources.get("allow-git").and_then(toml::Value::as_array) {
        if !allow_git.is_empty() {
            results.push(CheckResult {
                id: "RS-DENY-20".to_owned(),
                severity: Severity::Warn,
                title: "allow-git is non-empty".to_owned(),
                message: format!("`{}` has non-empty `[sources].allow-git`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        for entry in allow_git.iter().filter_map(toml::Value::as_str) {
            results.push(
                CheckResult {
                    id: "RS-DENY-20".to_owned(),
                    severity: Severity::Info,
                    title: "allow-git entry".to_owned(),
                    message: format!("`{}` allows git source `{entry}`.", config.rel_path),
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
pub(crate) use ::test_support::{build_fixture_deny_toml, set_allow_git_sources};
#[cfg(test)]
#[path = "rs_deny_20_allow_git_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_20_allow_git_inventory_tests;
