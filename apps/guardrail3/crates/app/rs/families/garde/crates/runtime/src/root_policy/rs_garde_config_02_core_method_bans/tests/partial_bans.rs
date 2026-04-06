use guardrail3_app_rs_family_garde_assertions::rs_garde_config_02_core_method_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn warns_when_bans_missing() {
    let root = temp_root("partial-garde-02");
    let mut clippy_toml = super::helpers::canonical_clippy_toml();
    for path in ["serde_json::from_slice", "serde_yaml::from_reader"] {
        clippy_toml =
            super::helpers::remove_clippy_ban_path(&clippy_toml, "disallowed-methods", path);
    }
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
    assertions::assert_missing(
        &results,
        "clippy.toml",
        "Missing core deserialization bans from `disallowed-methods`: serde_json::from_slice, serde_yaml::from_reader. Add these entries to `disallowed-methods` in clippy.toml.",
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
