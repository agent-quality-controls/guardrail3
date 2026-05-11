#[test]
fn empty_crawl_has_no_typecov_contracts() {
    let crawl = g3_workspace_crawl::G3WorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: Vec::new(),
    };
    g3ts_typecov_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}

#[test]
fn package_json_suffix_file_does_not_create_root() {
    let crawl = g3_workspace_crawl::G3WorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: vec![g3_workspace_crawl::G3WorkspaceEntry {
            path: g3_workspace_crawl::G3WorkspacePath {
                rel_path: "fixtures/my-package.json".to_owned(),
                abs_path: std::path::PathBuf::from("/tmp/fixtures/my-package.json"),
            },
            kind: g3_workspace_crawl::G3WorkspaceEntryKind::File,
            readable: true,
            ignore_state: g3_workspace_crawl::G3WorkspaceIgnoreState::Included,
        }],
    };

    g3ts_typecov_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}

#[test]
#[allow(
    clippy::disallowed_methods,
    clippy::panic,
    clippy::wildcard_enum_match_arm,
    reason = "test fixture writes via std::fs and asserts surface variants by panicking on mismatch"
)]
fn ignored_syncpack_config_is_missing() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(tempdir.path().join("package.json"), "{}\n").expect("write package manifest");
    std::fs::write(
        tempdir.path().join(".syncpackrc"),
        r#"{"versionGroups":[{"dependencies":["type-coverage"],"pinVersion":"8.20.0"}]}"#,
    )
    .expect("write syncpack config");

    let mut crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    for entry in &mut crawl.entries {
        if entry.path.rel_path == ".syncpackrc" {
            entry.ignore_state = g3_workspace_crawl::G3WorkspaceIgnoreState::Ignored;
        }
    }
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("typecov root should be discovered from package.json");

    match &contract.syncpack_config {
        g3ts_typecov_types::G3TsTypecovSyncpackSurfaceState::Missing { rel_path } => {
            assert_eq!(rel_path, ".syncpackrc");
        }
        other => panic!("expected ignored syncpack config to be missing, got {other:?}"),
    }
}

#[test]
#[allow(
    clippy::disallowed_methods,
    clippy::panic,
    reason = "test fixture writes via std::fs and asserts surface variants by panicking on mismatch"
)]
fn fail_open_type_coverage_script_preserves_or_separator() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(
        tempdir.path().join("package.json"),
        r#"{"scripts":{"typecov":"type-coverage --at-least 100 || true"}}"#,
    )
    .expect("write package manifest");

    let crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("typecov root should be discovered from package.json");
    let g3ts_typecov_types::G3TsTypecovPackageSurfaceState::Parsed { snapshot } = &contract.package
    else {
        panic!("expected parsed package surface");
    };

    let invocation = snapshot
        .script_tool_invocations
        .iter()
        .find(|invocation| invocation.executable == "type-coverage")
        .expect("type-coverage invocation should be preserved");
    assert_eq!(
        invocation.followed_by,
        Some(g3ts_typecov_types::G3TsTypecovPackageScriptCommandSeparator::Or)
    );
}

#[test]
#[allow(
    clippy::disallowed_methods,
    clippy::panic,
    reason = "test fixture writes via std::fs and asserts surface variants by panicking on mismatch"
)]
fn unsupported_typecov_script_is_preserved_as_parse_blocker() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(
        tempdir.path().join("package.json"),
        r#"{"scripts":{"typecov":"node scripts/typecov.js | tee log"}}"#,
    )
    .expect("write package manifest");

    let crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("typecov root should be discovered from package.json");
    let g3ts_typecov_types::G3TsTypecovPackageSurfaceState::Parsed { snapshot } = &contract.package
    else {
        panic!("expected parsed package surface");
    };

    let blocker = snapshot
        .script_parse_blockers
        .iter()
        .find(|blocker| blocker.script_name == "typecov")
        .expect("unsupported typecov script should be preserved as parse blocker");
    assert!(
        blocker.reason.contains("unsupported shell syntax"),
        "blocker should preserve parser reason: {}",
        blocker.reason
    );
}
