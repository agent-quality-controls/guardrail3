use crate::domain::report::Severity;

use super::super::test_support::{
    collected_facts, coverage_facts, coverage_input, dir_entry, project_tree,
};
use super::check;

#[test]
fn inventories_library_allowlist_when_present() {
    let facts = coverage_facts(Some("library"), true);
    let input = coverage_input(&facts, "packages/core/Cargo.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_library_allowlist_missing() {
    let facts = coverage_facts(Some("library"), false);
    let input = coverage_input(&facts, "packages/core/Cargo.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "library allowlist missing");
}

#[test]
fn package_profile_from_guardrail_config_is_respected() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["guardrail3.toml"])),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = coverage_input(&facts, "packages/core/Cargo.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-08");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("packages/core/Cargo.toml"));
}
