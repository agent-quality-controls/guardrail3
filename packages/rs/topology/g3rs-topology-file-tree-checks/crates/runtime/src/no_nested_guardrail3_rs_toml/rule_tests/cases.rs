use g3rs_topology_file_tree_checks_assertions::no_nested_guardrail3_rs_toml::rule as assertions;
use g3rs_topology_types::G3RsTopologyNestedGuardrail3RsTomlInput as NestedGuardrailInput;

use super::super::check;

#[test]
fn nested_guardrail3_rs_toml_under_root_outer_fires() {
    let input = NestedGuardrailInput {
        rel_dir: "crates/inner".to_owned(),
        guardrail3_rs_toml_rel_path: "crates/inner/guardrail3-rs.toml".to_owned(),
        outer_adopted_unit_rel: String::new(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested adopted Rust unit `crates/inner` is forbidden",
            "`crates/inner/guardrail3-rs.toml` declares a nested adopted Rust unit under `.`. Nested guardrail3 adoption breaks ownership routing. Remove `crates/inner/guardrail3-rs.toml`, or move the inner unit so it is not nested under `.`.",
            Some("crates/inner/guardrail3-rs.toml"),
            false,
        ),
    );
}

#[test]
fn nested_guardrail3_rs_toml_under_non_root_outer_mentions_that_outer() {
    let input = NestedGuardrailInput {
        rel_dir: "apps/outer/inner".to_owned(),
        guardrail3_rs_toml_rel_path: "apps/outer/inner/guardrail3-rs.toml".to_owned(),
        outer_adopted_unit_rel: "apps/outer".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested adopted Rust unit `apps/outer/inner` is forbidden",
            "`apps/outer/inner/guardrail3-rs.toml` declares a nested adopted Rust unit under `apps/outer`. Nested guardrail3 adoption breaks ownership routing. Remove `apps/outer/inner/guardrail3-rs.toml`, or move the inner unit so it is not nested under `apps/outer`.",
            Some("apps/outer/inner/guardrail3-rs.toml"),
            false,
        ),
    );
}
