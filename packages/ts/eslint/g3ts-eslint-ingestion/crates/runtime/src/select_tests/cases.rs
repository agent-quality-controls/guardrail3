use g3ts_eslint_ingestion_assertions::select as assertions;

#[test]
fn selects_root_config_by_official_precedence() {
    let root = super::helpers::fake_root();
    let crawl =
        super::helpers::crawl_with_entries(&root, &["eslint.config.ts", "eslint.config.js"]);

    let selected = super::super::select_active_root_eslint_config(&crawl)
        .expect("a root eslint config should be selected");

    assertions::assert_selected_rel_path(selected, "eslint.config.js");
}
