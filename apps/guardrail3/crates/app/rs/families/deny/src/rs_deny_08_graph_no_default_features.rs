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
        _ => results.push(CheckResult {
            id: "RS-DENY-08".to_owned(),
            severity: Severity::Error,
            title: "graph no-default-features must be false".to_owned(),
            message: format!(
                "`{}` must set `[graph].no-default-features = false`.",
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
        id: "RS-DENY-08".to_owned(),
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
#[path = "rs_deny_08_graph_no_default_features_tests/mod.rs"]
mod tests;
