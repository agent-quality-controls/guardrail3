/// Asserts that ingesting `crawl` produces `expected` fmt contracts.
///
/// # Panics
///
/// Panics when the produced contract count does not match `expected`.
pub fn assert_ingests_contract_count(
    crawl: &g3_workspace_crawl::G3RsWorkspaceCrawl,
    expected: usize,
) {
    let input = g3ts_fmt_ingestion_runtime::ingest_for_config_checks(crawl);
    assert_eq!(
        input.contracts.len(),
        expected,
        "fmt contract count should match"
    );
}
