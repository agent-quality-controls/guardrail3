#[test]
fn empty_crawl_has_no_spelling_contracts() {
    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: Vec::new(),
    };
    g3ts_spelling_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}

#[test]
fn invalid_json_cspell_config_is_parse_error() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(tempdir.path().join("package.json"), "{}\n").expect("write package manifest");
    std::fs::write(tempdir.path().join("cspell.json"), "{\n").expect("write invalid cspell config");

    let crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("spelling root should be discovered from package.json");

    match &contract.cspell_config {
        g3ts_spelling_types::G3TsSpellingConfigSurfaceState::ParseError { rel_path, reason } => {
            assert_eq!(rel_path, "cspell.json");
            assert!(
                reason.contains("EOF"),
                "parse reason should preserve JSON parser detail: {reason}"
            );
        }
        other => panic!("expected cspell config parse error, got {other:?}"),
    }
}

#[test]
fn yaml_cspell_config_is_delegated_by_existence() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(tempdir.path().join("package.json"), "{}\n").expect("write package manifest");
    std::fs::write(tempdir.path().join("cspell.yaml"), "not: [parsed\n")
        .expect("write yaml cspell config");

    let crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("spelling root should be discovered from package.json");

    match &contract.cspell_config {
        g3ts_spelling_types::G3TsSpellingConfigSurfaceState::Parsed { rel_path } => {
            assert_eq!(rel_path, "cspell.yaml");
        }
        other => panic!("expected cspell yaml config to be delegated, got {other:?}"),
    }
}

#[test]
fn ignored_cspell_config_does_not_create_root() {
    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: vec![g3_workspace_crawl::G3RsWorkspaceEntry {
            path: g3_workspace_crawl::G3RsWorkspacePath {
                rel_path: "ignored/cspell.json".to_owned(),
                abs_path: std::path::PathBuf::from("/tmp/ignored/cspell.json"),
            },
            kind: g3_workspace_crawl::G3RsWorkspaceEntryKind::File,
            readable: true,
            ignore_state: g3_workspace_crawl::G3RsWorkspaceIgnoreState::Ignored,
        }],
    };

    g3ts_spelling_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}

#[test]
fn package_json_suffix_file_does_not_create_root() {
    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: ".".into(),
        entries: vec![g3_workspace_crawl::G3RsWorkspaceEntry {
            path: g3_workspace_crawl::G3RsWorkspacePath {
                rel_path: "fixtures/my-package.json".to_owned(),
                abs_path: std::path::PathBuf::from("/tmp/fixtures/my-package.json"),
            },
            kind: g3_workspace_crawl::G3RsWorkspaceEntryKind::File,
            readable: true,
            ignore_state: g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included,
        }],
    };

    g3ts_spelling_ingestion_assertions::run::assert_ingests_contract_count(&crawl, 0);
}

#[test]
fn ignored_syncpack_config_is_missing() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(tempdir.path().join("package.json"), "{}\n").expect("write package manifest");
    std::fs::write(
        tempdir.path().join(".syncpackrc"),
        r#"{"versionGroups":[{"dependencies":["cspell"],"pinVersion":"8.20.0"}]}"#,
    )
    .expect("write syncpack config");

    let mut crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    for entry in &mut crawl.entries {
        if entry.path.rel_path == ".syncpackrc" {
            entry.ignore_state = g3_workspace_crawl::G3RsWorkspaceIgnoreState::Ignored;
        }
    }
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("spelling root should be discovered from package.json");

    match &contract.syncpack_config {
        g3ts_spelling_types::G3TsSpellingSyncpackSurfaceState::Missing { rel_path } => {
            assert_eq!(rel_path, ".syncpackrc");
        }
        other => panic!("expected ignored syncpack config to be missing, got {other:?}"),
    }
}

#[test]
fn fail_open_cspell_script_preserves_or_separator() {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    std::fs::write(
        tempdir.path().join("package.json"),
        r#"{"scripts":{"spellcheck":"cspell . || true"}}"#,
    )
    .expect("write package manifest");

    let crawl =
        g3_workspace_crawl::crawl_any_root(tempdir.path()).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    let contract = input
        .contracts
        .first()
        .expect("spelling root should be discovered from package.json");
    let g3ts_spelling_types::G3TsSpellingPackageSurfaceState::Parsed { snapshot } =
        &contract.package
    else {
        panic!("expected parsed package surface");
    };

    let invocation = snapshot
        .script_tool_invocations
        .iter()
        .find(|invocation| invocation.executable == "cspell")
        .expect("cspell invocation should be preserved");
    assert_eq!(
        invocation.followed_by,
        Some(g3ts_spelling_types::G3TsSpellingPackageScriptCommandSeparator::Or)
    );
}
