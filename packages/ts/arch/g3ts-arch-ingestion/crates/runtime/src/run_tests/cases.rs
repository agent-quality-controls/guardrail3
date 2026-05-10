use std::fs;

use g3_workspace_crawl::crawl_any_root as crawl;
use g3ts_arch_types::G3TsArchManifestState;

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "test fixture builder uses std::fs directly to set up a temp workspace; not production code"
)]
#[expect(
    clippy::indexing_slicing,
    reason = "test asserts on a known-length fixture vector and any out-of-bounds is a test bug we want to surface"
)]
#[expect(
    clippy::panic,
    reason = "test guards a `let-else` on a fixture state that must be `Parsed`; panic surfaces fixture mismatch"
)]
fn ingest_collects_declared_facade_and_existing_entrypoints() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace for arch ingestion");
    fs::write(
        tempdir.path().join("package.json"),
        r#"{"exports":{".":"./src/index.ts"}}"#,
    )
    .expect("write root package.json fixture for arch ingestion");
    fs::create_dir_all(tempdir.path().join("src/components"))
        .expect("create source tree fixture for arch ingestion");
    fs::write(
        tempdir.path().join("src/index.ts"),
        "export { Button } from \"./components/button\";\n",
    )
    .expect("write facade entrypoint fixture at src/index.ts");
    fs::write(
        tempdir.path().join("src/components/button.ts"),
        "export const Button = 1;\n",
    )
    .expect("write nested source fixture at src/components/button.ts");

    let crawl = crawl(tempdir.path()).expect("crawl temporary workspace for arch ingestion");
    let config = crate::run::ingest_for_config_checks(&crawl);
    let file_tree = crate::run::ingest_for_file_tree_checks(&crawl);
    let source = crate::run::ingest_for_source_checks(&crawl);

    let G3TsArchManifestState::Parsed { snapshot } = config.manifest else {
        panic!("expected parsed manifest");
    };
    assert_eq!(
        snapshot.declared_entrypoints.len(),
        1,
        "expected one declared facade"
    );
    assert_eq!(snapshot.declared_entrypoints[0].rel_path, "src/index.ts");
    assert_eq!(
        file_tree.existing_entrypoints,
        vec!["src/index.ts".to_owned()],
        "expected exact existing entrypoint inventory"
    );
    assert_eq!(source.len(), 1, "expected one source input batch");
    assert_eq!(source[0].facades.len(), 1, "expected one facade file");
}
