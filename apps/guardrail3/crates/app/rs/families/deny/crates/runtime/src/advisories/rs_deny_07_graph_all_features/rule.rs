use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_graph, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(graph) = section(config, "graph") else {
        push_missing(config, results);
        return;
    };
    let (expected_all_features, _) = expected_graph();
    match graph.get("all-features").and_then(toml::Value::as_bool) {
        Some(value) if value == expected_all_features => {}
        _ => results.push(CheckResult::from_parts(
            "RS-DENY-07".to_owned(),
            Severity::Error,
            "graph all-features must be true".to_owned(),
            format!(
                "`{}` must set `[graph].all-features = true`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        )),
    }
}

fn push_missing(config: &crate::facts::DenyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        "RS-DENY-07".to_owned(),
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
