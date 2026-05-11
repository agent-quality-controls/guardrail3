/// Assert that ingesting `crawl` yields exactly `expected` contracts.
///
/// # Panics
///
/// Panics when the ingested contract count differs from `expected`.
pub fn assert_ingests_contract_count(
    crawl: &g3_workspace_crawl::G3WorkspaceCrawl,
    expected: usize,
) {
    let input = g3ts_spelling_ingestion_runtime::ingest_for_config_checks(crawl);
    assert_eq!(
        input.contracts.len(),
        expected,
        "spelling contract count should match"
    );
}
