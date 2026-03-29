use guardrail3_app_rs_family_garde_assertions::rs_garde_06_additional_method_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_when_all_bans_present() {
    let root = temp_root("golden-garde-06");
    let clippy_toml = super::super::canonical_clippy_toml();
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("clippy.toml", clippy_toml.as_str()),
        ],
        root.clone(),
    );
    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-06 findings: {findings:#?}"
    );
    assertions::assert_inventory(
        &results,
        "clippy.toml",
        "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
