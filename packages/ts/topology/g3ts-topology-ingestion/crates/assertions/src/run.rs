use g3ts_topology_types::G3TsTopologyFileTreeChecksInput;

/// Asserts that `input` has no nested `guardrail3-ts.toml` facts.
///
/// # Panics
///
/// Panics when any nested fact is present.
pub fn assert_no_nested(input: &G3TsTopologyFileTreeChecksInput) {
    assert!(
        input.nested_guardrail3_ts_tomls.is_empty(),
        "expected no nested guardrail3-ts.toml facts, got {:#?}",
        input.nested_guardrail3_ts_tomls
    );
}

/// Asserts that `input` contains a nested-fact whose toml relative path
/// equals `expected_toml_rel_path`.
///
/// # Panics
///
/// Panics when no fact matches the expected relative path.
pub fn assert_nested_at(input: &G3TsTopologyFileTreeChecksInput, expected_toml_rel_path: &str) {
    let matched = input
        .nested_guardrail3_ts_tomls
        .iter()
        .any(|fact| fact.toml_rel_path == expected_toml_rel_path);
    assert!(
        matched,
        "expected nested guardrail3-ts.toml fact at `{expected_toml_rel_path}`, got {:#?}",
        input.nested_guardrail3_ts_tomls
    );
}
