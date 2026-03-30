use guardrail3_app_rs_family_garde_assertions::rs_garde_03_extractor_type_bans as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn warns_when_bans_missing() {
    let root = temp_root("partial-garde-03");
    let mut clippy_toml = super::super::canonical_clippy_toml();
    for path in ["axum::extract::Path", "axum_extra::extract::TypedHeader"] {
        clippy_toml = super::super::remove_clippy_ban_path(&clippy_toml, "disallowed-types", path);
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
    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-03 findings: {findings:#?}"
    );
    assertions::assert_missing(
        &results,
        "clippy.toml",
        "Missing garde extractor bans from `disallowed-types`: axum::extract::Path, axum_extra::extract::TypedHeader.",
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
