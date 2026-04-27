use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_graph;
use crate::support::findings::error;

const ID: &str = "g3rs-deny/graph-no-default-features";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(graph) = deny.graph.as_ref() else {
        results.push(error(
            ID,
            "[graph] section missing",
            format!("`{deny_rel_path}` must contain `[graph]` coverage settings."),
            deny_rel_path,
        ));
        return;
    };
    let (_, expected_no_default_features) = expected_graph();
    match graph.no_default_features {
        Some(value) if value == expected_no_default_features => {}
        _ => results.push(error(
            ID,
            "graph no-default-features must be false",
            format!("`{deny_rel_path}` must set `[graph].no-default-features = false`."),
            deny_rel_path,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
