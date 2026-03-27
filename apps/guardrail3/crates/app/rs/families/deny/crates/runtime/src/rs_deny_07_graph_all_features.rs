use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_graph, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(graph) = section(config, "graph") else {
        push_missing(config, results);
        return;
    };
    let (expected_all_features, _) = expected_graph();
    match graph.get("all-features").and_then(toml::Value::as_bool) {
        Some(value) if value == expected_all_features => {}
        _ => results.push(CheckResult {
            id: "RS-DENY-07".to_owned(),
            severity: Severity::Error,
            title: "graph all-features must be true".to_owned(),
            message: format!(
                "`{}` must set `[graph].all-features = true`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

fn push_missing(config: &super::facts::DenyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: "RS-DENY-07".to_owned(),
        severity: Severity::Error,
        title: "[graph] section missing".to_owned(),
        message: format!(
            "`{}` must contain `[graph]` coverage settings.",
            config.rel_path
        ),
        file: Some(config.rel_path.clone()),
        line: None,
        inventory: false,
    });
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
pub(crate) use crate::{config_facts};
#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, copy_fixture, remove_section, remove_section_key, set_section_bool, write_file};
#[cfg(test)]
#[cfg(test)]
#[path = "rs_deny_07_graph_all_features_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_07_graph_all_features_tests;
