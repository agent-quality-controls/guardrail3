use guardrail3_app_rs_family_garde_assertions::rs_garde_06_additional_method_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_additional_ban_only_warns_for_owned_root() {
    let root = temp_root("multi-garde-06");
    let root_clippy = super::super::canonical_clippy_toml();
    let local_clippy = super::super::remove_clippy_ban_path(
        &super::super::canonical_clippy_toml(),
        "disallowed-methods",
        "serde_json::Deserializer::from_reader",
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "vendor"], &[])),
            ("apps", dir_entry(&["root"], &[])),
            (
                "apps/root",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor", dir_entry(&["lib"], &[])),
            ("vendor/lib", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
        ],
        vec![
            (
                "apps/root/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"root\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("apps/root/clippy.toml", root_clippy.as_str()),
            (
                "vendor/lib/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"vendored\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("vendor/lib/clippy.toml", local_clippy.as_str()),
        ],
        root.clone(),
    );
    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        2,
        "unexpected RS-GARDE-06 findings: {findings:#?}"
    );
    assertions::assert_inventory(
        &results,
        "apps/root/clippy.toml",
        "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
    );
    assertions::assert_missing(
        &results,
        "vendor/lib/clippy.toml",
        "Missing additional garde deserialization bans from `disallowed-methods`: serde_json::Deserializer::from_reader.",
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
