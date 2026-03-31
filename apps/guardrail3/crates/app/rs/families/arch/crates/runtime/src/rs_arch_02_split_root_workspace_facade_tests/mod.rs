use guardrail3_app_rs_family_mapper::DirEntry;

#[test]
fn split_library_root_must_keep_workspace_and_package() {
    let tree = super::super::ProjectTree::new(
        std::path::PathBuf::from("/tmp/arch-tests"),
        std::collections::BTreeMap::from([
            (
                String::new(),
                DirEntry::new(vec!["packages".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "packages".to_owned(),
                DirEntry::new(vec!["shared".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "packages/shared".to_owned(),
                DirEntry::new(
                    vec!["crates".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/shared/crates".to_owned(),
                DirEntry::new(vec!["core".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "packages/shared/crates/core".to_owned(),
                DirEntry::new(
                    vec!["src".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/shared/crates/core/src".to_owned(),
                DirEntry::new(Vec::new(), vec!["lib.rs".to_owned()], Vec::new(), Vec::new()),
            ),
        ]),
        std::collections::BTreeMap::from([
            (
                "packages/shared/Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"crates/core\"]\n".to_owned(),
            ),
            (
                "packages/shared/crates/core/Cargo.toml".to_owned(),
                "[package]\nname = \"shared-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n".to_owned(),
            ),
            (
                "packages/shared/crates/core/src/lib.rs".to_owned(),
                "pub struct CoreType;\n".to_owned(),
            ),
        ]),
    );

    let results = super::super::check_test_tree(&tree);
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-02"));
}
