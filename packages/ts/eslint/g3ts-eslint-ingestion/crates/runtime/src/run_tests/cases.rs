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

#[test]
fn returns_unreadable_when_selected_root_config_is_unreadable() {
    let root = super::helpers::fake_eslint_workspace();
    let crawl = super::helpers::crawl_with_entries(&root, &["src/index.ts"]);
    let unreadable = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: crawl.root_abs_path,
        entries: vec![g3_workspace_crawl::G3RsWorkspaceEntry {
            path: g3_workspace_crawl::G3RsWorkspacePath {
                rel_path: "eslint.config.mjs".to_owned(),
                abs_path: root.path().join("eslint.config.mjs"),
            },
            kind: g3_workspace_crawl::G3RsWorkspaceEntryKind::File,
            ignore_state: g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included,
            readable: false,
        }],
    };

    let input = super::super::ingest_for_config_checks(&unreadable);

    assertions::assert_unreadable(&input, "eslint.config.mjs");
}
