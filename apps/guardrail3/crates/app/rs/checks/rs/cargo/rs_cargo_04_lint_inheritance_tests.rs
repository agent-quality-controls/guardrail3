use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, entry, member_input, tree};
use super::check;

#[test]
fn members_inheriting_workspace_lints_inventory_pass() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api", "domain"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/domain", entry(&[], &["Cargo.toml"])),
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
            (
                "crates/domain/Cargo.toml",
                r#"
                    [package]
                    name = "domain"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);

    let mut api_results = Vec::new();
    check(&member_input(&facts, "crates/api"), &mut api_results);
    assert_eq!(api_results.len(), 1);
    let api_result = &api_results[0];
    assert_eq!(api_result.id, "RS-CARGO-04");
    assert!(api_result.inventory);
    assert_eq!(api_result.severity, Severity::Info);
    assert_eq!(api_result.title, "workspace lints inherited");
    assert_eq!(
        api_result.message,
        "api: `[lints] workspace = true` inherits workspace lint policy"
    );
    assert_eq!(api_result.file.as_deref(), Some("crates/api/Cargo.toml"));

    let mut domain_results = Vec::new();
    check(&member_input(&facts, "crates/domain"), &mut domain_results);
    assert_eq!(domain_results.len(), 1);
    let domain_result = &domain_results[0];
    assert_eq!(domain_result.id, "RS-CARGO-04");
    assert!(domain_result.inventory);
    assert_eq!(domain_result.severity, Severity::Info);
    assert_eq!(domain_result.title, "workspace lints inherited");
    assert_eq!(
        domain_result.message,
        "domain: `[lints] workspace = true` inherits workspace lint policy"
    );
    assert_eq!(
        domain_result.file.as_deref(),
        Some("crates/domain/Cargo.toml")
    );
}

#[test]
fn non_inheriting_member_errors() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
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

                    [workspace.lints.rust]
                    warnings = "deny"
                "#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints.rust]
                    warnings = "allow"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&member_input(&facts, "crates/api"), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CARGO-04");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "workspace lints not inherited");
    assert_eq!(
        result.message,
        "crates/api: missing `[lints] workspace = true` in member Cargo.toml"
    );
    assert_eq!(result.file.as_deref(), Some("crates/api/Cargo.toml"));
}
