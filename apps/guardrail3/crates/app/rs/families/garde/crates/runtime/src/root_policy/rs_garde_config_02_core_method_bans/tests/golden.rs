use guardrail3_app_rs_family_garde_assertions::rs_garde_config_02_core_method_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_when_all_bans_present() {
    let root = temp_root("golden-garde-02");
    let clippy_toml = super::helpers::canonical_clippy_toml();
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("clippy.toml", clippy_toml.as_str()),
        ],
        root.clone(),
    );
    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-CONFIG-02 findings: {findings:#?}"
    );
    assertions::assert_inventory(
        &results,
        "clippy.toml",
        "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.",
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
