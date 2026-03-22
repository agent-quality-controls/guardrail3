use super::collect;
use super::super::inputs::WorkspaceMembersSetInput;
use super::super::test_support::{entry, tree};

#[test]
fn workspace_member_set_input_is_bound_from_facts() {
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

    let facts = collect(&tree).expect("workspace facts");
    let input = WorkspaceMembersSetInput::from_facts(&facts);
    assert_eq!(input.workspace.rel_path, "Cargo.toml");
    assert!(input.declared_members.contains("crates/api"));
    assert!(input.discovered_members.contains("crates/domain"));
}
