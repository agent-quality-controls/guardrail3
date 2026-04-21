#[test]
fn returns_missing_when_root_jscpd_is_absent() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_jscpd_ingestion_assertions::run::assert_root_missing(&input);
}

#[test]
fn parses_root_jscpd_when_present() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(
        tempdir.path(),
        ".jscpd.json",
        r#"{ "threshold": 0, "minTokens": 50, "absolute": true, "format": ["typescript"] }"#,
    );

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_jscpd_ingestion_assertions::run::assert_root_parsed(&input, ".jscpd.json");
}

#[test]
fn surfaces_parse_error_for_invalid_root_jscpd() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(tempdir.path(), ".jscpd.json", "{ invalid ");

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_jscpd_ingestion_assertions::run::assert_root_parse_error(&input, ".jscpd.json");
}
