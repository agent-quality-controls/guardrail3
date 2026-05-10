#[test]
fn returns_not_package_manager_root_without_pnpm_markers() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(tempdir.path(), ".npmrc", "engine-strict=true\n");
    super::helpers::seed_cargo_manifest_if_missing(tempdir.path());

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_npmrc_ingestion_assertions::run::assert_root_not_package_manager_root(&input);
}

#[test]
fn returns_missing_when_root_npmrc_is_absent() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(tempdir.path(), "pnpm-lock.yaml", "lockfileVersion: '9.0'\n");
    super::helpers::seed_cargo_manifest_if_missing(tempdir.path());

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_npmrc_ingestion_assertions::run::assert_root_missing(&input);
}

#[test]
fn parses_root_npmrc_when_package_manager_markers_exist() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(
        tempdir.path(),
        "pnpm-workspace.yaml",
        "packages:\n  - apps/*\n",
    );
    super::helpers::write(
        tempdir.path(),
        ".npmrc",
        "strict-peer-dependencies=true\nengine-strict=true\n",
    );
    super::helpers::seed_cargo_manifest_if_missing(tempdir.path());

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_npmrc_ingestion_assertions::run::assert_root_parsed(&input, ".npmrc");
}

#[test]
fn surfaces_parse_error_for_invalid_root_npmrc() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    super::helpers::write(tempdir.path(), "pnpm-lock.yaml", "lockfileVersion: '9.0'\n");
    super::helpers::write(tempdir.path(), ".npmrc", "not-a-setting\n");
    super::helpers::seed_cargo_manifest_if_missing(tempdir.path());

    let crawl = g3_workspace_crawl::crawl(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_npmrc_ingestion_assertions::run::assert_root_parse_error(&input, ".npmrc");
}
