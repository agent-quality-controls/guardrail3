#[test]
fn empty_crawl_has_no_fmt_contracts() {
    let crawl = g3_workspace_crawl::G3WorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: Vec::new(),
    };
    g3ts_fmt_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}
