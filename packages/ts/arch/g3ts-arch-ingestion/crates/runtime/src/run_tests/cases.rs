use std::fs;

use g3_workspace_crawl::crawl;

#[test]
fn ingest_collects_declared_facade_and_source_tree() {
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
    let config = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingest arch config facts from temporary workspace");
    let file_tree = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("ingest arch file-tree facts from temporary workspace");
    let source = crate::run::ingest_for_source_checks(&crawl)
        .expect("ingest arch source facts from temporary workspace");

    let g3ts_arch_types::G3TsArchManifestState::Parsed { snapshot } = config.manifest else {
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
    assert!(
        file_tree.source_tree.is_some(),
        "expected source tree metrics for populated src tree"
    );
    assert_eq!(source.len(), 1, "expected one source input batch");
    assert_eq!(source[0].facades.len(), 1, "expected one facade file");
}

#[test]
fn ingest_counts_direct_child_of_src_as_depth_one() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace for depth fixture");
    fs::create_dir_all(tempdir.path().join("src/core"))
        .expect("create direct child source directory fixture");
    fs::write(
        tempdir.path().join("src/core/value.ts"),
        "export const value = 1;\n",
    )
    .expect("write direct child source file fixture");

    let crawl = crawl(tempdir.path()).expect("crawl temporary workspace for depth fixture");
    let file_tree = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("ingest arch file-tree facts for direct-child depth fixture");

    assert_eq!(
        file_tree.source_tree.map(|tree| tree.max_depth),
        Some(1),
        "direct child of src should count as depth one"
    );
}

#[test]
fn ingest_ignores_test_and_example_subtrees_in_structure_counts() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for ignored-subtree fixture");
    fs::create_dir_all(tempdir.path().join("src/core"))
        .expect("create core source directory fixture");
    fs::create_dir_all(tempdir.path().join("src/tests"))
        .expect("create tests source directory fixture");
    fs::create_dir_all(tempdir.path().join("src/__tests__"))
        .expect("create __tests__ source directory fixture");
    fs::create_dir_all(tempdir.path().join("src/examples"))
        .expect("create examples source directory fixture");
    fs::write(
        tempdir.path().join("src/core/value.ts"),
        "export const value = 1;\n",
    )
    .expect("write core source file fixture");
    fs::write(
        tempdir.path().join("src/tests/spec.ts"),
        "export const testValue = 1;\n",
    )
    .expect("write tests source file fixture");
    fs::write(
        tempdir.path().join("src/__tests__/spec.ts"),
        "export const nestedTestValue = 1;\n",
    )
    .expect("write __tests__ source file fixture");
    fs::write(
        tempdir.path().join("src/examples/example.ts"),
        "export const exampleValue = 1;\n",
    )
    .expect("write examples source file fixture");

    let crawl =
        crawl(tempdir.path()).expect("crawl temporary workspace for ignored-subtree fixture");
    let file_tree = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("ingest arch file-tree facts for ignored-subtree fixture");
    let source_tree = file_tree
        .source_tree
        .expect("source tree should exist for ignored-subtree fixture");

    assert_eq!(
        source_tree.max_sibling_dir_count, 1,
        "only real source directories should count toward sibling directory pressure"
    );
    assert_eq!(
        source_tree.max_sibling_code_file_count, 1,
        "only real source files should count toward sibling file pressure"
    );
}

#[test]
fn ingest_counts_deep_source_tree_past_threshold() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace for deep-tree fixture");
    fs::create_dir_all(tempdir.path().join("src/a/b/c/d"))
        .expect("create deep source directory fixture");
    fs::write(
        tempdir.path().join("src/a/b/c/d/value.ts"),
        "export const value = 1;\n",
    )
    .expect("write deep source file fixture");

    let crawl = crawl(tempdir.path()).expect("crawl temporary workspace for deep-tree fixture");
    let file_tree = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("ingest arch file-tree facts for deep-tree fixture");

    assert_eq!(
        file_tree.source_tree.map(|tree| tree.max_depth),
        Some(4),
        "deep tree should surface max depth past the wave-one threshold"
    );
}
