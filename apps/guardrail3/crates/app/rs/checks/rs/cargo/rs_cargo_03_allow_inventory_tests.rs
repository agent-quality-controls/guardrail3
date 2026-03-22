use crate::domain::modules::canonical;
use crate::domain::report::Severity;

use super::super::inputs::WorkspaceCargoInput;
use super::super::lint_support::EXPECTED_CLIPPY_ALLOW;
use super::super::test_support::{collected_facts, entry, has_result, tree};
use super::check;

#[test]
fn approved_allow_deviations_are_inventoried() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [workspace]
                members = []
                resolver = "2"

                [workspace.package]
                edition = "2024"

                [workspace.lints.clippy]
                all = { level = "deny", priority = -1 }
                pedantic = { level = "deny", priority = -1 }
                cargo = { level = "deny", priority = -1 }
                nursery = { level = "deny", priority = -1 }
                missing_docs_in_private_items = "allow"
                module_name_repetitions = "allow"
            "#,
        )],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    let rule_results: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-CARGO-03")
        .collect();
    assert!(rule_results.len() >= 2);
    assert!(rule_results.iter().all(|result| result.inventory));
    assert!(rule_results.iter().all(|result| result.severity == Severity::Info));
    assert!(has_result(&results, "RS-CARGO-03", |result| {
        result.inventory
            && result.severity == Severity::Info
            && result.title == "allow inventory: `missing_docs_in_private_items`"
            && result.message == "`missing_docs_in_private_items` is explicitly allowed."
    }));
    assert!(has_result(&results, "RS-CARGO-03", |result| {
        result.inventory
            && result.severity == Severity::Info
            && result.title == "allow inventory: `module_name_repetitions`"
            && result.message == "`module_name_repetitions` is explicitly allowed."
    }));
}

#[test]
fn expected_allow_inventory_matches_canonical_module() {
    let parsed: toml::Value =
        toml::from_str(canonical::CARGO_LINTS.content).expect("canonical cargo lints should parse");
    let clippy = parsed
        .get("workspace")
        .and_then(|value| value.get("lints"))
        .and_then(|value| value.get("clippy"))
        .and_then(toml::Value::as_table)
        .expect("canonical clippy lints table");

    for lint_name in EXPECTED_CLIPPY_ALLOW {
        assert!(
            clippy.contains_key(*lint_name),
            "canonical cargo lints missing clippy allow lint `{lint_name}`",
        );
    }
}
