use guardrail3_app_rs_family_mapper::DirEntry;

#[test]
fn flat_library_over_threshold_must_split() {
    let deps = (0..13)
        .map(|index| format!("dep{index} = \"1\"\n"))
        .collect::<String>();
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
                    vec!["src".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/shared/src".to_owned(),
                DirEntry::new(Vec::new(), vec!["lib.rs".to_owned()], Vec::new(), Vec::new()),
            ),
        ]),
        std::collections::BTreeMap::from([
            (
                "packages/shared/Cargo.toml".to_owned(),
                format!(
                    "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{deps}"
                ),
            ),
            (
                "packages/shared/src/lib.rs".to_owned(),
                "pub fn facade() -> u8 { 1 }\n".to_owned(),
            ),
        ]),
    );

    let results = super::super::check_test_tree(&tree);
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-01"));
}
