use guardrail3_app_rs_family_garde_assertions::rs_garde_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn evaluates_workspace_and_standalone_package_roots() {
    let root = temp_root("rs-garde-01-multi");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["vendor"],
                    &["Cargo.toml", "clippy.toml", "guardrail3.toml"],
                ),
            ),
            ("vendor", dir_entry(&["lib", "tool"], &[])),
            (
                "vendor/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor/lib/src", dir_entry(&[], &["lib.rs"])),
            (
                "vendor/tool",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor/tool/src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
"#,
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "vendor/lib/Cargo.toml",
                r#"[package]
name = "lib"
version = "0.1.0"
[dependencies]
garde = "0.22"
"#,
            ),
            (
                "vendor/lib/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("vendor/lib/src/lib.rs", "pub fn ok() {}"),
            (
                "vendor/tool/Cargo.toml",
                r#"[package]
name = "tool"
version = "0.1.0"
[dependencies]
serde = "1"
"#,
            ),
            (
                "vendor/tool/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("vendor/tool/src/main.rs", "fn main() {}"),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                file: Some("Cargo.toml"),
                inventory: Some(false),
                title: Some("garde dependency missing"),
                message_contains: Some("workspace root"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Info),
                file: Some("vendor/lib/Cargo.toml"),
                inventory: Some(true),
                title: Some("garde dependency found"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                file: Some("vendor/tool/Cargo.toml"),
                inventory: Some(false),
                title: Some("garde dependency missing"),
                message_contains: Some("standalone package root"),
                ..Default::default()
            },
        ],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
