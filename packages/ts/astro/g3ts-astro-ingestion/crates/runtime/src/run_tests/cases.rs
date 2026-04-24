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
fn ignored_package_json_with_astro_dependency_does_not_create_an_app_root() {
    let root = super::helpers::fake_astro_workspace();

    std::fs::create_dir_all(root.path().join("node_modules/fake-astro"))
        .expect("fake astro package directory should be created");
    std::fs::write(
        root.path().join("node_modules/fake-astro/package.json"),
        "{\n  \"dependencies\": {\n    \"astro\": \"1.0.0\"\n  }\n}\n",
    )
    .expect("fake astro package manifest should be written");

    let crawl = g3_workspace_crawl::G3RsWorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: vec![super::helpers::ignored_entry(
            &root,
            "node_modules/fake-astro/package.json",
        )],
    };

    let config_input = super::super::ingest_for_config_checks(&crawl);
    let file_tree_input = super::super::ingest_for_file_tree_checks(&crawl);

    assert!(
        config_input.integration_contracts.is_empty(),
        "ignored package should not create config contracts: {config_input:?}"
    );
    assert!(
        config_input.eslint_contracts.is_empty(),
        "ignored package should not create eslint contracts: {config_input:?}"
    );
    assert!(
        file_tree_input.app_roots.is_empty(),
        "ignored package should not create file-tree roots: {file_tree_input:?}"
    );
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
            "src/pages/index.astro",
            "src/pages/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_eq!(input.integration_contracts.len(), 1, "unexpected inputs: {input:?}");
    assert_eq!(input.eslint_contracts.len(), 1, "unexpected inputs: {input:?}");

    let integration = &input.integration_contracts[0];
    assert_eq!(integration.content_mode, G3TsAstroContentMode::BuildCollections);
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
                snapshot
                    .astro_source_plugins
                    .iter()
                    .any(|plugin| plugin == "astro"),
                "astro plugin missing from astro lane: {snapshot:?}"
            );
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
            assert!(
                snapshot
                    .ts_source_effective_route_scoped_pipeline_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-authored-content-fs-read"),
                "route-scoped pipeline options missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_accepts_endpoint_only_scope_options_for_route_scoped_pipeline_rules() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    const isTsx = String(_filePath).endsWith('.tsx');
    return {
      plugins: {
        astro: {},
        "astro-pipeline": {},
      },
      rules: {
        "astro-pipeline/no-authored-content-fs-read": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
      },
      languageOptions: { parserOptions: { projectService: true, jsx: isTsx } },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint runtime should be rewritten");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.ts_source_effective_route_scoped_pipeline_rules.len(),
                4,
                "endpoint-scoped rules should count as effective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_malformed_scope_options_for_route_scoped_pipeline_rules() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    const isTsx = String(_filePath).endsWith('.tsx');
    return {
      plugins: {
        astro: {},
        "astro-pipeline": {},
      },
      rules: {
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: [1] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: [1] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: [1] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: [1] }],
      },
      languageOptions: { parserOptions: { projectService: true, jsx: isTsx } },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint runtime should be rewritten");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot
                    .ts_source_effective_route_scoped_pipeline_rules
                    .is_empty(),
                "malformed numeric scope options must not count as effective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn plain_astro_app_without_content_still_requires_pipeline_linting() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = &input.integration_contracts[0];
    let eslint = &input.eslint_contracts[0];

    assert_eq!(integration.content_mode, G3TsAstroContentMode::None);
    assert!(integration.requires_source_pipeline_linting);
    assert!(eslint.requires_source_pipeline_linting);
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
fn file_tree_ingestion_collects_route_mdx_inputs() {
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
}

#[test]
fn nested_astro_app_root_uses_its_own_package_and_nearest_eslint_surface() {
    let root = super::helpers::fake_astro_workspace();

    std::fs::create_dir_all(root.path().join("apps/landing/src/pages"))
        .expect("nested pages directory should be created");
    std::fs::create_dir_all(root.path().join("apps/landing/src/content"))
        .expect("nested content directory should be created");
    std::fs::write(
        root.path().join("apps/landing/package.json"),
        "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\",\n    \"eslint-plugin-astro\": \"1.0.0\",\n    \"eslint-plugin-astro-pipeline\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check\"\n  }\n}\n",
    )
    .expect("nested package manifest should be written");
    std::fs::write(
        root.path().join("apps/landing/astro.config.mjs"),
        "import react from '@astrojs/react';\nexport default { output: 'static', integrations: [react()] };\n",
    )
    .expect("nested astro config should be written");
    std::fs::write(
        root.path().join("apps/landing/src/content.config.ts"),
        "export default {};\n",
    )
    .expect("nested content config should be written");
    std::fs::write(
        root.path().join("apps/landing/src/pages/index.astro"),
        "---\n---\n<main />\n",
    )
    .expect("nested route should be written");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "apps/landing/package.json",
            "apps/landing/astro.config.mjs",
            "apps/landing/src/content.config.ts",
            "apps/landing/src/pages/index.astro",
            "eslint.config.mjs",
            "node_modules/eslint/index.js",
        ],
    );

    let config_input = super::super::ingest_for_config_checks(&crawl);
    assert_eq!(config_input.integration_contracts.len(), 1, "unexpected config inputs: {config_input:?}");
    assert_eq!(config_input.eslint_contracts.len(), 1, "unexpected config inputs: {config_input:?}");

    let integration = &config_input.integration_contracts[0];
    assert_eq!(integration.app_root_rel_path, "apps/landing");
    match &integration.package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => {
            assert_eq!(snapshot.rel_path, "apps/landing/package.json");
        }
        other => panic!("expected parsed nested package state, got {other:?}"),
    }

    match &config_input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert_eq!(snapshot.rel_path, "eslint.config.mjs");
        }
        other => panic!("expected parsed inherited eslint state, got {other:?}"),
    }

    let file_tree_input = super::super::ingest_for_file_tree_checks(&crawl);
    assert_eq!(file_tree_input.app_roots.len(), 1, "unexpected file-tree inputs: {file_tree_input:?}");
    assert_eq!(file_tree_input.app_roots[0].app_root_rel_path, "apps/landing");
    assert_eq!(
        file_tree_input.app_roots[0].astro_config_rel_path.as_deref(),
        Some("apps/landing/astro.config.mjs")
    );
}

#[test]
fn package_only_astro_root_is_not_lost_when_another_app_has_astro_config() {
    let root = super::helpers::fake_astro_workspace();

    std::fs::create_dir_all(root.path().join("apps/landing/src/pages"))
        .expect("landing pages directory should be created");
    std::fs::create_dir_all(root.path().join("apps/docs/src/pages"))
        .expect("docs pages directory should be created");

    std::fs::write(
        root.path().join("apps/landing/package.json"),
        "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check\"\n  }\n}\n",
    )
    .expect("landing package manifest should be written");
    std::fs::write(
        root.path().join("apps/landing/astro.config.mjs"),
        "export default { output: 'static' };\n",
    )
    .expect("landing astro config should be written");
    std::fs::write(
        root.path().join("apps/landing/src/pages/index.astro"),
        "---\n---\n<main />\n",
    )
    .expect("landing route should be written");

    std::fs::write(
        root.path().join("apps/docs/package.json"),
        "{\n  \"dependencies\": {\n    \"astro\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check\"\n  }\n}\n",
    )
    .expect("docs package manifest should be written");
    std::fs::write(
        root.path().join("apps/docs/src/pages/index.astro"),
        "---\n---\n<main />\n",
    )
    .expect("docs route should be written");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "apps/landing/package.json",
            "apps/landing/astro.config.mjs",
            "apps/landing/src/pages/index.astro",
            "apps/docs/package.json",
            "apps/docs/src/pages/index.astro",
        ],
    );

    let config_input = super::super::ingest_for_config_checks(&crawl);
    let app_roots = config_input
        .integration_contracts
        .iter()
        .map(|contract| contract.app_root_rel_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(app_roots, vec!["apps/docs", "apps/landing"]);
}

#[test]
fn ignored_eslint_probe_does_not_count_as_a_live_lane() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(filePath) {
    return String(filePath).endsWith(".astro");
  }

  async calculateConfigForFile(filePath) {
    const isTsx = String(filePath).endsWith(".tsx");
    return {
      plugins: {
        astro: {},
        "astro-pipeline": {},
      },
      rules: {
        "astro-pipeline/no-authored-content-fs-read": "error",
        "astro-pipeline/no-authored-content-glob": "error",
        "astro-pipeline/no-direct-astro-content-in-routes": "error",
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": "error",
      },
      languageOptions: { parserOptions: { projectService: true, jsx: isTsx } },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint runtime should be overwritten");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(!snapshot.astro_source_probe_present, "ignored astro probe should not count: {snapshot:?}");
            assert!(snapshot.ts_source_probe_present, "ts lane should stay present: {snapshot:?}");
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}
