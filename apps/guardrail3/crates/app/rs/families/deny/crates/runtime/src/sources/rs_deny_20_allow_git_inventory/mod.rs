use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::section;
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(sources) = section(config, "sources") else {
        return;
    };
    if let Some(allow_git) = sources.get("allow-git").and_then(toml::Value::as_array) {
        if !allow_git.is_empty() {
            results.push(CheckResult::from_parts(
                "RS-DENY-20".to_owned(),
                Severity::Warn,
                "allow-git is non-empty".to_owned(),
                format!("`{}` has non-empty `[sources].allow-git`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
        for entry in allow_git.iter().filter_map(toml::Value::as_str) {
            if entry.trim().is_empty() {
                results.push(CheckResult::from_parts(
                    "RS-DENY-20".to_owned(),
                    Severity::Error,
                    "allow-git entry must be non-empty".to_owned(),
                    format!(
                        "`{}` has blank `[sources].allow-git` entry.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-20".to_owned(),
                    Severity::Info,
                    "allow-git entry".to_owned(),
                    format!("`{}` allows git source `{entry}`.", config.rel_path),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
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

mod rs_deny_20_allow_git_inventory_tests;
