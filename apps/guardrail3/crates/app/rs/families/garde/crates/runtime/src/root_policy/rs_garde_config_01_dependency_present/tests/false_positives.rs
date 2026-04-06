use guardrail3_app_rs_family_garde_assertions::rs_garde_config_01_dependency_present as assertions;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn stays_quiet_for_helper_crate_without_garde_markers() {
    let root = temp_root("rs-garde-01-helper-quiet");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[package]
name = "helper"
version = "0.1.0"

[dependencies]
serde = "1"
"#,
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "src/lib.rs",
                r#"
pub fn helper_name() -> &'static str {
    "helper"
}
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
