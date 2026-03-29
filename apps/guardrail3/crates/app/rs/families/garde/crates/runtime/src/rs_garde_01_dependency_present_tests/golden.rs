use guardrail3_app_rs_family_garde_assertions::rs_garde_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_when_garde_dependency_present() {
    let root = temp_root("rs-garde-01-golden");

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
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            ("src/main.rs", "fn main() {}"),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("garde dependency found"),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            message: Some(
                "garde is present in `Cargo.toml` for this workspace root. Garde-specific boundary checks are active.",
            ),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
