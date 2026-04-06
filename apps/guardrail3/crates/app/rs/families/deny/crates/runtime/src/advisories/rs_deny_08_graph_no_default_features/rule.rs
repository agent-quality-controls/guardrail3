use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_graph, section};
use crate::inputs::ConfigDenyInput;

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
            "RS-DENY-CONFIG-05".to_owned(),
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

fn push_missing(config: &crate::facts::DenyConfigFacts, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        "RS-DENY-CONFIG-05".to_owned(),
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
