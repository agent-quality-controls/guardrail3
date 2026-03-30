use guardrail3_app_rs_family_garde_assertions::rs_garde_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn evaluates_sibling_workspace_roots() {
    let root = temp_root("rs-garde-01-multi");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps", "vendor"], &["guardrail3.toml"]),
            ),
            ("apps", dir_entry(&["root"], &[])),
            (
                "apps/root",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/root/src", dir_entry(&[], &["main.rs"])),
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
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "apps/root/Cargo.toml",
                r#"[workspace]
members = []
[package]
name = "root"
version = "0.1.0"
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
"#,
            ),
            (
                "apps/root/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            (
                "apps/root/src/main.rs",
                r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct RootBoundary {
    value: String,
}
"#,
            ),
            (
                "vendor/lib/Cargo.toml",
                r#"[workspace]
members = []
[package]
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
                r#"[workspace]
members = []
[package]
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
            (
                "vendor/tool/src/main.rs",
                r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct ToolBoundary {
    value: String,
}

fn main() {}
"#,
            ),
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
                file: Some("apps/root/Cargo.toml"),
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
                message_contains: Some("workspace root"),
                ..Default::default()
            },
        ],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
