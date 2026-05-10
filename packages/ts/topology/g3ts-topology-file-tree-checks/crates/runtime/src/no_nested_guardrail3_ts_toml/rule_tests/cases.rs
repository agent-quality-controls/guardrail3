use g3ts_topology_file_tree_checks_assertions::no_nested_guardrail3_ts_toml::rule as assertions;
use g3ts_topology_types::G3TsTopologyNestedGuardrail3TsTomlInput;
use guardrail3_check_types::G3CheckResult;

use super::super::check;

#[test]
fn nested_input_emits_error_finding() {
    let input = G3TsTopologyNestedGuardrail3TsTomlInput {
        rel_dir: "packages/inner".to_owned(),
        toml_rel_path: "packages/inner/guardrail3-ts.toml".to_owned(),
        parent_unit_rel: String::new(),
    };

    let mut results: Vec<G3CheckResult> = Vec::new();
    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        &assertions::finding(
            assertions::Severity::Error,
            "Nested `guardrail3-ts.toml` at `packages/inner` is forbidden",
            "`packages/inner/guardrail3-ts.toml` declares a nested adopted TS unit marker under `.`. Nested adoption breaks the upward-walk routing assumption. Remove the inner `packages/inner/guardrail3-ts.toml`, or move it so it is not nested under `.`.",
            Some("packages/inner/guardrail3-ts.toml"),
            false,
        ),
    );
}

#[test]
fn nested_input_under_named_parent_emits_finding_referencing_that_parent() {
    let input = G3TsTopologyNestedGuardrail3TsTomlInput {
        rel_dir: "packages/inner".to_owned(),
        toml_rel_path: "packages/inner/guardrail3-ts.toml".to_owned(),
        parent_unit_rel: "apps/outer".to_owned(),
    };

    let mut results: Vec<G3CheckResult> = Vec::new();
    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        &assertions::finding(
            assertions::Severity::Error,
            "Nested `guardrail3-ts.toml` at `packages/inner` is forbidden",
            "`packages/inner/guardrail3-ts.toml` declares a nested adopted TS unit marker under `apps/outer`. Nested adoption breaks the upward-walk routing assumption. Remove the inner `packages/inner/guardrail3-ts.toml`, or move it so it is not nested under `apps/outer`.",
            Some("packages/inner/guardrail3-ts.toml"),
            false,
        ),
    );
}
