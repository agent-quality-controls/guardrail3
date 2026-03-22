use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, entry, member_input, tree};
use super::check;

#[test]
fn older_member_edition_is_warned() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["legacy"], &[])),
            ("crates/legacy", entry(&[], &["Cargo.toml"])),
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
                "crates/legacy/Cargo.toml",
                r#"
                    [package]
                    name = "legacy"
                    edition = "2021"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);
    let input = member_input(&facts, "crates/legacy");
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CARGO-09");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "member edition older than workspace");
    assert_eq!(
        result.message,
        "crates/legacy sets edition `2021` while workspace uses `2024`."
    );
}
