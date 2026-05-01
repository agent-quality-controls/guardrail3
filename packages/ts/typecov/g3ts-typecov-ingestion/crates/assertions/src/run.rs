pub fn assert_ingests_contract_count(
    crawl: &g3_workspace_crawl::G3RsWorkspaceCrawl,
    expected: usize,
) {
    let input = g3ts_typecov_ingestion_runtime::ingest_for_config_checks(crawl);
    assert_eq!(
        input.contracts.len(),
        expected,
        "typecov contract count should match"
    );
}
