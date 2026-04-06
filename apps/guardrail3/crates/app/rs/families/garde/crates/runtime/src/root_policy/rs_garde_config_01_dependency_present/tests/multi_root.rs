use guardrail3_app_rs_family_garde_assertions::rs_garde_config_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn evaluates_sibling_workspace_roots() {
    let root = temp_root("rs-garde-01-multi");

    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["root", "lib", "tool"], &[])),
            (
                "apps/root",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/root/src", dir_entry(&[], &["main.rs"])),
            (
                "apps/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/lib/src", dir_entry(&[], &["lib.rs"])),
            (
                "apps/tool",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/tool/src", dir_entry(&[], &["main.rs"])),
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
                "apps/lib/Cargo.toml",
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
                "apps/lib/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("apps/lib/src/lib.rs", "pub fn ok() {}"),
            (
                "apps/tool/Cargo.toml",
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
                "apps/tool/clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            (
                "apps/tool/src/main.rs",
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

    let results = super::helpers::run_family(&tree);
    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                file: Some("apps/root/Cargo.toml"),
                inventory: Some(false),
                title: Some("garde dependency missing"),
                message_contains: Some("Add `garde` to `[dependencies]`"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Info),
                file: Some("apps/lib/Cargo.toml"),
                inventory: Some(true),
                title: Some("garde dependency found"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                file: Some("apps/tool/Cargo.toml"),
                inventory: Some(false),
                title: Some("garde dependency missing"),
                message_contains: Some("Add `garde` to `[dependencies]`"),
                ..Default::default()
            },
        ],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
