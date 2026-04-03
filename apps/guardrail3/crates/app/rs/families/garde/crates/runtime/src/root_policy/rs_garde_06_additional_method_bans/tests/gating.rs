use guardrail3_app_rs_family_garde_assertions::rs_garde_06_additional_method_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn ignores_workspace_when_garde_missing() {
    let root = temp_root("gating-garde-06");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
            ),
            ("clippy.toml", "disallowed-methods = []"),
        ],
        root.clone(),
    );
    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert!(
        findings.is_empty(),
        "expected no RS-GARDE-06 findings: {findings:#?}"
    );
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
