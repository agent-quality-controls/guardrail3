use g3ts_topology_types::G3TsTopologyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Runs every file-tree rule against `input` and returns aggregated results.
#[must_use]
pub fn check(input: &G3TsTopologyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for nested in &input.nested_guardrail3_ts_tomls {
        crate::no_nested_guardrail3_ts_toml::check(nested, &mut results);
    }

    results
}
