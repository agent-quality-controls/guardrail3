use guardrail3_app_rs_family_mapper::DirEntry;

#[test]
fn external_roots_must_go_through_facade_package() {
    let tree = super::super::ProjectTree::new(
        std::path::PathBuf::from("/tmp/arch-tests"),
        std::collections::BTreeMap::from([
            (
                String::new(),
                DirEntry::new(vec!["packages".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "packages".to_owned(),
                DirEntry::new(
                    vec!["shared".to_owned(), "consumer".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/shared".to_owned(),
                DirEntry::new(
                    vec!["src".to_owned(), "crates".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/shared/src".to_owned(),
                DirEntry::new(Vec::new(), vec!["lib.rs".to_owned()], Vec::new(), Vec::new()),
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
            (
                "packages/consumer".to_owned(),
                DirEntry::new(
                    vec!["src".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "packages/consumer/src".to_owned(),
                DirEntry::new(Vec::new(), vec!["lib.rs".to_owned()], Vec::new(), Vec::new()),
            ),
        ]),
        std::collections::BTreeMap::from([
            (
                "packages/shared/Cargo.toml".to_owned(),
                "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"crates/core\"]\n".to_owned(),
            ),
            (
                "packages/shared/src/lib.rs".to_owned(),
                "pub use shared_core::CoreType;\n".to_owned(),
            ),
            (
                "packages/shared/crates/core/Cargo.toml".to_owned(),
                "[package]\nname = \"shared-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n".to_owned(),
            ),
            (
                "packages/shared/crates/core/src/lib.rs".to_owned(),
                "pub struct CoreType;\n".to_owned(),
            ),
            (
                "packages/consumer/Cargo.toml".to_owned(),
                "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshared-core = { path = \"../shared/crates/core\" }\n".to_owned(),
            ),
            (
                "packages/consumer/src/lib.rs".to_owned(),
                "pub struct Consumer;\n".to_owned(),
            ),
        ]),
    );

    let results = super::super::check_test_tree(&tree);
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-04"));
}
