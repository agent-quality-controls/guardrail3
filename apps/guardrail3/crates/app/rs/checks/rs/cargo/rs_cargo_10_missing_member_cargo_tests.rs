use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, entry, members_set_input, tree};
use super::check;

#[test]
fn declared_member_without_cargo_toml_is_warned() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api", "missing"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/missing", entry(&[], &[])),
        ],
        &[
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["crates/*"]
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"
                "#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&members_set_input(&facts), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CARGO-10");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "declared workspace member missing Cargo.toml");
    assert_eq!(
        result.message,
        "`crates/missing` is declared in `[workspace].members` but no `Cargo.toml` was discovered there."
    );
    assert_eq!(result.file.as_deref(), Some("Cargo.toml"));
}
