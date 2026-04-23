use g3ts_astro_types::{
    G3TsAstroContentMode, G3TsAstroEslintSurfaceState, G3TsAstroPackageSurfaceState,
};

#[test]
fn config_ingestion_returns_empty_for_non_astro_roots() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(&root, &["src/pages/index.ts"]);

    let input = super::super::ingest_for_config_checks(&crawl);

    assert!(input.integration_contracts.is_empty(), "unexpected config inputs: {input:?}");
    assert!(input.eslint_contracts.is_empty(), "unexpected eslint inputs: {input:?}");
}

#[test]
fn config_ingestion_collects_package_and_eslint_contracts_for_astro_roots() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "eslint.config.mjs",
            "src/pages/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_eq!(input.integration_contracts.len(), 1, "unexpected inputs: {input:?}");
    assert_eq!(input.eslint_contracts.len(), 1, "unexpected inputs: {input:?}");

    let integration = &input.integration_contracts[0];
    assert_eq!(integration.content_mode, G3TsAstroContentMode::BuildCollections);
    assert!(integration.requires_render_validator);
    assert!(integration.requires_source_pipeline_linting);

    match &integration.package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot.dev_dependencies.iter().any(|dependency| dependency == "astro"),
                "astro dependency missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .script_bodies
                    .iter()
                    .any(|(_, body)| body.contains("astro check")),
                "astro check script missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed package state, got {other:?}"),
    }

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot.ts_source_plugins.iter().any(|plugin| plugin == "astro"),
                "astro plugin missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_plugins
                    .iter()
                    .any(|plugin| plugin == "astro-pipeline"),
                "astro pipeline plugin missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_error_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-authored-content-fs-read"),
                "astro pipeline rules missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_marks_unreadable_package_surface() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: vec![
            super::helpers::unreadable_entry(&root, "package.json"),
            super::helpers::unreadable_entry(&root, "astro.config.mjs"),
        ],
    };

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_eq!(input.integration_contracts.len(), 1, "unexpected inputs: {input:?}");
    match &input.integration_contracts[0].package {
        G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. } => {
            assert_eq!(rel_path, "package.json");
        }
        other => panic!("expected unreadable package state, got {other:?}"),
    }
}

#[test]
fn file_tree_ingestion_collects_route_mdx_and_cross_root_loader_inputs() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "src/pages/index.ts",
            "src/pages/about.mdx",
            "src/lib/kit-taxonomy.ts",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);

    assert_eq!(input.app_roots.len(), 1, "unexpected file-tree inputs: {input:?}");
    assert_eq!(
        input.build_collection_roots.len(),
        1,
        "unexpected build-collection roots: {input:?}"
    );
    assert_eq!(input.route_markdown_pages.len(), 1, "unexpected markdown pages: {input:?}");
    assert_eq!(
        input.route_markdown_pages[0].rel_path,
        "src/pages/about.mdx",
        "unexpected markdown page: {input:?}"
    );
    assert_eq!(
        input.cross_root_side_loaders.len(),
        0,
        "unexpected side loaders: {input:?}"
    );
}
