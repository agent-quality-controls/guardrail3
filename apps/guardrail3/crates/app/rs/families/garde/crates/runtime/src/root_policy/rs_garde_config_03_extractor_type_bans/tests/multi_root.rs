use guardrail3_app_rs_family_garde_assertions::rs_garde_config_03_extractor_type_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_extractor_ban_only_warns_for_owned_root() {
    let root = temp_root("multi-garde-03");
    let root_clippy = super::helpers::canonical_clippy_toml();
    let local_clippy = super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-types",
        "axum::extract::Multipart",
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["root", "lib"], &[])),
            ("apps/root", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
            ("apps/lib", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
        ],
        vec![
            (
                "apps/root/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"root\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("apps/root/clippy.toml", root_clippy.as_str()),
            (
                "apps/lib/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"vendored\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("apps/lib/clippy.toml", local_clippy.as_str()),
        ],
        root.clone(),
    );
    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        2,
        "unexpected RS-GARDE-CONFIG-03 findings: {findings:#?}"
    );
    assertions::assert_inventory(
        &results,
        "apps/root/clippy.toml",
        "All required Axum extractor bans are present in the covering clippy configuration.",
    );
    assertions::assert_missing(
        &results,
        "apps/lib/clippy.toml",
        "Missing extractor type bans from `disallowed-types`: axum::extract::Multipart. Add these entries to `disallowed-types` in clippy.toml.",
    );
    std::fs::remove_dir_all(&root).expect("remove temp root");
}
