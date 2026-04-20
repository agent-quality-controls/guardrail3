use g3ts_eslint_ingestion_assertions::run as assertions;

#[test]
fn returns_missing_when_no_root_config_exists() {
    let root = super::helpers::fake_eslint_workspace();
    let crawl = super::helpers::crawl_with_entries(&root, &["src/index.ts"]);

    let input = super::super::ingest_for_config_checks(&crawl);

    assertions::assert_missing(&input);
}

#[test]
fn returns_parsed_document_for_selected_root_config() {
    let root = super::helpers::fake_eslint_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "eslint.config.mjs",
            "src/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assertions::assert_parsed_rel_path(&input, "eslint.config.mjs");
}
