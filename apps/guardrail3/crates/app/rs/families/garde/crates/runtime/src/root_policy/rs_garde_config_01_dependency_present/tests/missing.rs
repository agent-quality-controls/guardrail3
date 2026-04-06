use guardrail3_app_rs_family_garde_assertions::rs_garde_config_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_garde_dependency_missing() {
    let root = temp_root("rs-garde-01-missing");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["main.rs"])),
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
                "src/main.rs",
                r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct Boundary {
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
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("garde dependency missing"),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            message_contains: Some("Add `garde` to `[dependencies]`"),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
