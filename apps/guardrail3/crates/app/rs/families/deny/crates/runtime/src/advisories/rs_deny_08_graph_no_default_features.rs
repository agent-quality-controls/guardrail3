use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_graph, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(graph) = section(config, "graph") else {
        push_missing(config, results);
        return;
    };
    let (_, expected_no_default_features) = expected_graph();
    match graph
        .get("no-default-features")
        .and_then(toml::Value::as_bool)
    {
        Some(value) if value == expected_no_default_features => {}
        _ => results.push(CheckResult::from_parts(
            "RS-DENY-08".to_owned(),
            Severity::Error,
            "graph no-default-features must be false".to_owned(),
            format!(
                "`{}` must set `[graph].no-default-features = false`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
    }
}

fn push_missing(config: &super::facts::DenyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        "RS-DENY-08".to_owned(),
        Severity::Error,
        "[graph] section missing".to_owned(),
        format!(
            "`{}` must contain `[graph]` coverage settings.",
            config.rel_path
        ),
        Some(config.rel_path.clone()),
        None,
        false,
    ));
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
pub(crate) use crate::config_facts;
#[cfg(test)]
pub(crate) use ::test_support::{
    build_fixture_deny_toml, copy_fixture, remove_section, remove_section_key, set_section_bool,
    write_file,
};
#[cfg(test)]
#[path = "rs_deny_08_graph_no_default_features_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_08_graph_no_default_features_tests;
