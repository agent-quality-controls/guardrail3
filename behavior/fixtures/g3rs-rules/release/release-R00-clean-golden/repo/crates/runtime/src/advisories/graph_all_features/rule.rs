use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_graph;
use crate::support::findings::error;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/graph-all-features";

/// Runs the rule and appends any findings to `results`.
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
    let (expected_all_features, _) = expected_graph();
    match graph.all_features {
        Some(value) if value == expected_all_features => {}
        _ => results.push(error(
            ID,
            "graph all-features must be true",
            format!("`{deny_rel_path}` must set `[graph].all-features = true`."),
            deny_rel_path,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
