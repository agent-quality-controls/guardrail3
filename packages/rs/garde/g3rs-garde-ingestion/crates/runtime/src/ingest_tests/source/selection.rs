#[test]
fn ingests_root_scoped_ast_input() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/lib.rs"),
        "use garde::Validate;\n#[derive(Validate)] struct Input;\n",
    );
    super::write(root.join("tests/ignored.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("tests/fixtures/root_ignored.rs"), "fn ignored() {}\n");
    super::write(root.join("tests.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/support_test.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/support_tests.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/helpers_tests/mod.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/__tests__/ignored.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/test.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/tests.rs"), "#[test]\nfn ignored() {}\n");
    super::write(root.join("src/main.rs"), "fn main() {}\n");
    super::write(root.join("src/tests/fixtures/broken.rs"), "fn broken( {\n");
    super::write(root.join("build.rs"), "fn broken( {\n");
    super::write(root.join("examples/demo.rs"), "fn broken( {\n");
    super::write(root.join("crates/member/src/lib.rs"), "fn nested() {}\n");
    super::write(
        root.join("src/member/Cargo.toml"),
        "[package]\nname = \"nested\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("src/member/src/lib.rs"), "fn nested_root() {}\n");

    let crawl = super::crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

    assert_eq!(input.applicability, g3rs_garde_types::G3RsGardeApplicability::Active);
    assert_eq!(
        input
            .guardrail_toml
            .as_ref()
            .expect("active garde input should include guardrail3.toml")
            .rel_path,
        "guardrail3.toml"
    );
    assert_eq!(
        input
            .source_files
            .iter()
            .map(|file| file.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec!["src/lib.rs", "src/main.rs"],
        "{input:#?}"
    );
}
