use g3ts_astro_types::{
    G3TsAstroContentMode, G3TsAstroEslintSurfaceState, G3TsAstroPackageSurfaceState,
    G3TsAstroPolicySurfaceState,
};
use std::process::Command;

#[test]
fn config_ingestion_returns_empty_for_non_astro_roots() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(&root, &["src/pages/index.ts"]);

    let input = super::super::ingest_for_config_checks(&crawl);

    assert!(
        input.integration_contracts.is_empty(),
        "unexpected config inputs: {input:?}"
    );
    assert!(
        input.eslint_contracts.is_empty(),
        "unexpected eslint inputs: {input:?}"
    );
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
    std::fs::write(
        root.path().join("src/pages/card.tsx"),
        "export function Card() { return null; }\n",
    )
    .expect("tsx source probe should be written");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.html",
            "src/pages/index.ts",
            "src/pages/card.tsx",
            "src/lib/content/index.ts",
            "src/content/posts/example.mdx",
            "src/components/mdx/index.tsx",
            "src/lib/metadata/index.ts",
            "src/lib/json-ld/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_eq!(
        input.integration_contracts.len(),
        1,
        "unexpected inputs: {input:?}"
    );
    assert_eq!(
        input.eslint_contracts.len(),
        1,
        "unexpected inputs: {input:?}"
    );

    let integration = &input.integration_contracts[0];
    assert_eq!(
        integration.content_mode,
        G3TsAstroContentMode::BuildCollections
    );

    match &integration.package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot
                    .dev_dependencies
                    .iter()
                    .any(|dependency| dependency == "astro"),
                "astro dependency missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .script_bodies
                    .iter()
                    .any(|(_, body)| body.contains("astro check")),
                "astro check script missing: {snapshot:?}"
            );
            assert!(
                snapshot.script_commands.iter().any(|command| {
                    command.script_name == "check"
                        && command.executable == "astro"
                        && command.args.first().is_some_and(|arg| arg == "check")
                }),
                "typed astro check command missing: {snapshot:?}"
            );
            assert!(
                snapshot.safely_runs_astro_check,
                "safe astro check fact missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed package state, got {other:?}"),
    }

    match &integration.syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
            assert!(
                snapshot.source_covers_package_manifest,
                "syncpack source should cover package.json: {snapshot:?}"
            );
            assert!(
                snapshot.missing_required_stack_pins.is_empty(),
                "required pin facts should be satisfied: {snapshot:?}"
            );
            assert!(
                snapshot.missing_forbidden_bans.is_empty(),
                "forbidden ban facts should be satisfied: {snapshot:?}"
            );
        }
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }

    match &integration.astro_policy {
        G3TsAstroPolicySurfaceState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.profile.as_deref(),
                Some("strict-static-content"),
                "strict Astro policy profile missing: {snapshot:?}"
            );
            assert_eq!(
                snapshot.content_routes,
                vec!["src/pages/**/*.astro".to_owned()],
                "content route policy mismatch: {snapshot:?}"
            );
            assert_eq!(
                snapshot.content_root.as_deref(),
                Some("src/content"),
                "content root policy mismatch: {snapshot:?}"
            );
            assert_eq!(
                snapshot.content_adapters,
                vec!["src/lib/content".to_owned()],
                "content adapter policy mismatch: {snapshot:?}"
            );
            assert_eq!(
                snapshot.required_collections,
                vec!["landing".to_owned()],
                "required collection policy mismatch: {snapshot:?}"
            );
            assert_eq!(
                snapshot.collection_fields.get("landing"),
                Some(&vec!["title".to_owned(), "description".to_owned()]),
                "collection field policy mismatch: {snapshot:?}"
            );
            assert_eq!(
                integration.approved_surface_sources.content_adapter,
                vec!["src/lib/content/index.ts".to_owned()],
                "content adapter source path mismatch: {integration:?}"
            );
            assert_eq!(
                integration
                    .approved_surface_sources
                    .content_adapter_astro_content,
                vec!["src/lib/content/index.ts".to_owned()],
                "astro:content adapter source path mismatch: {integration:?}"
            );
            assert_eq!(
                integration.approved_surface_sources.mdx_component_maps,
                vec!["src/components/mdx/index.tsx".to_owned()],
                "mdx component map source path mismatch: {integration:?}"
            );
            assert!(
                integration
                    .approved_surface_sources
                    .missing_mdx_component_maps
                    .is_empty(),
                "mdx component map source should exist: {integration:?}"
            );
            assert_eq!(
                integration.approved_surface_sources.metadata_helpers,
                vec!["src/lib/metadata/index.ts".to_owned()],
                "metadata helper source path mismatch: {integration:?}"
            );
            assert!(
                integration
                    .approved_surface_sources
                    .missing_metadata_helpers
                    .is_empty(),
                "metadata helper source should exist: {integration:?}"
            );
            assert_eq!(
                integration.approved_surface_sources.json_ld_helpers,
                vec!["src/lib/json-ld/index.ts".to_owned()],
                "json-ld helper source path mismatch: {integration:?}"
            );
            assert!(
                integration
                    .approved_surface_sources
                    .missing_json_ld_helpers
                    .is_empty(),
                "json-ld helper source should exist: {integration:?}"
            );
        }
        other => panic!("expected parsed astro policy, got {other:?}"),
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
                snapshot
                    .ts_source_plugins
                    .iter()
                    .any(|plugin| plugin == "astro"),
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
                    .ts_source_error_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-authored-content-imports"),
                "authored-content import rule missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_route_scoped_pipeline_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-authored-content-fs-read"),
                "route-scoped pipeline options missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_route_scoped_pipeline_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/require-approved-content-adapter-in-routes"),
                "approved content adapter option missing: {snapshot:?}"
            );
            assert_eq!(
                snapshot.ts_source_effective_content_adapter_modules,
                vec!["src/lib/content/**/*".to_owned()],
                "approved content adapter modules mismatch: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_data_pipeline_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-content-data-modules-in-routes"),
                "content-data pipeline options missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_source_pipeline_rules
                    .iter()
                    .any(|rule| rule == "astro-pipeline/no-authored-content-imports"),
                "content-source pipeline options missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .astro_source_plugins
                    .iter()
                    .any(|plugin| plugin == "i18next"),
                "inline public content plugin missing from astro lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_error_rules
                    .iter()
                    .any(|rule| rule == "i18next/no-literal-string"),
                "inline public content rule missing: {snapshot:?}"
            );
            assert!(
                snapshot
                    .astro_source_effective_inline_public_content_rules
                    .iter()
                    .any(|rule| rule == "i18next/no-literal-string"),
                "inline public content policy missing from astro lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_inline_public_content_rules
                    .iter()
                    .any(|rule| rule == "i18next/no-literal-string"),
                "inline public content policy missing from ts lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .tsx_source_effective_inline_public_content_rules
                    .iter()
                    .any(|rule| rule == "i18next/no-literal-string"),
                "inline public content policy missing from tsx lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .mdx_content_plugins
                    .iter()
                    .any(|plugin| plugin == "mdx"),
                "mdx plugin missing from mdx lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .mdx_content_error_rules
                    .iter()
                    .any(|rule| rule == "mdx/remark"),
                "mdx remark rule missing from mdx lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .mdx_content_effective_mdx_component_map_rules
                    .iter()
                    .any(|rule| {
                        rule == "astro-pipeline/mdx-component-imports-from-approved-map"
                    }),
                "mdx component-map rule missing from mdx lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_metadata_helper_rules
                    .iter()
                    .any(|rule| {
                        rule == "astro-pipeline/require-approved-metadata-helper-in-routes"
                    }),
                "metadata helper rule missing from ts lane: {snapshot:?}"
            );
            assert!(
                snapshot
                    .tsx_source_effective_json_ld_helper_rules
                    .iter()
                    .any(|rule| {
                        rule == "astro-pipeline/require-approved-json-ld-helper-in-routes"
                    }),
                "json-ld helper rule missing from tsx lane: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_ignores_rust_policy_filename_for_astro_policy() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::rename(
        root.path().join("guardrail3-ts.toml"),
        root.path().join("guardrail3-rs.toml"),
    )
    .expect("test policy file should be renamed to the old Rust filename");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-rs.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_eq!(
        input.integration_contracts.len(),
        1,
        "astro app should still be detected: {input:?}"
    );
    assert_eq!(
        input.integration_contracts[0].astro_policy,
        G3TsAstroPolicySurfaceState::Missing {
            rel_path: "guardrail3-ts.toml".to_owned()
        },
        "G3TS Astro policy must use the TypeScript filename"
    );
}

#[test]
fn config_ingestion_rejects_helper_rules_with_eslint_options_outside_policy_sources() {
    for (case_name, original_option, replacement_option, assertion) in [
        (
            "mdx component map",
            r#"mdxContentGlobs: ["src/content/**/*.mdx"], approvedMdxComponentModules: ["src/components/mdx/**/*"]"#,
            r#"mdxContentGlobs: ["src/content/**/*.mdx"], approvedMdxComponentModules: ["src/components/private/**/*"]"#,
            "mdx",
        ),
        (
            "metadata helper",
            r#"approvedMetadataHelperModules: ["src/lib/metadata/**/*"], approvedContentAdapterModules: ["src/lib/content/**/*"]"#,
            r#"approvedMetadataHelperModules: ["src/lib/private-metadata/**/*"], approvedContentAdapterModules: ["src/lib/content/**/*"]"#,
            "metadata",
        ),
        (
            "json-ld helper",
            r#"approvedJsonLdHelperModules: ["src/lib/json-ld/**/*"]"#,
            r#"approvedJsonLdHelperModules: ["src/lib/private-json-ld/**/*"]"#,
            "json-ld",
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        let eslint_runtime =
            std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
                .expect("fake eslint runtime should be readable")
                .replace(original_option, replacement_option);
        std::fs::write(
            root.path().join("node_modules/eslint/index.js"),
            eslint_runtime,
        )
        .expect("fake eslint runtime should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "guardrail3-ts.toml",
                "eslint.config.mjs",
                "src/pages/index.astro",
                "src/pages/index.ts",
                "src/pages/card.tsx",
                "src/content/posts/example.mdx",
                "src/components/mdx/index.tsx",
                "src/lib/metadata/index.ts",
                "src/lib/json-ld/index.ts",
                "src/lib/content/index.ts",
                "node_modules/eslint/index.js",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);

        match &input.eslint_contracts[0].config {
            G3TsAstroEslintSurfaceState::Parsed { snapshot } => match assertion {
                "mdx" => assert!(
                    snapshot
                        .mdx_content_effective_mdx_component_map_rules
                        .is_empty(),
                    "{case_name} must make MDX component-map rule ineffective: {snapshot:?}"
                ),
                "metadata" => assert!(
                    snapshot
                        .ts_source_effective_metadata_helper_rules
                        .is_empty(),
                    "{case_name} must make metadata helper rule ineffective: {snapshot:?}"
                ),
                "json-ld" => assert!(
                    snapshot.ts_source_effective_json_ld_helper_rules.is_empty(),
                    "{case_name} must make JSON-LD helper rule ineffective: {snapshot:?}"
                ),
                other => panic!("unknown assertion case: {other}"),
            },
            other => panic!("expected parsed eslint state for {case_name}, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_rejects_helper_rules_missing_one_configured_policy_surface() {
    let root = super::helpers::fake_astro_workspace();
    let policy = std::fs::read_to_string(root.path().join("guardrail3-ts.toml"))
        .expect("guardrail policy should be readable")
        .replace(
            r#"metadata_helpers = ["src/lib/metadata"]"#,
            r#"metadata_helpers = ["src/lib/metadata", "src/lib/metadata-extra"]"#,
        );
    std::fs::write(root.path().join("guardrail3-ts.toml"), policy)
        .expect("guardrail policy should be rewritten");
    std::fs::create_dir_all(root.path().join("src/lib/metadata-extra"))
        .expect("extra metadata helper directory should be created");
    std::fs::write(
        root.path().join("src/lib/metadata-extra/index.ts"),
        "export const extraMetadata = {};\n",
    )
    .expect("extra metadata helper source should be written");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "src/pages/card.tsx",
            "src/lib/content/index.ts",
            "src/lib/metadata/index.ts",
            "src/lib/metadata-extra/index.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = &input.integration_contracts[0];
    assert!(
        integration
            .approved_surface_sources
            .missing_metadata_helpers
            .is_empty(),
        "all configured metadata helper sources should exist: {integration:?}"
    );

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot
                    .ts_source_effective_metadata_helper_rules
                    .is_empty(),
                "omitting one configured metadata helper surface from ESLint options must make the rule ineffective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_helper_rules_with_eslint_mdx_content_globs_outside_content() {
    let root = super::helpers::fake_astro_workspace();
    let eslint_runtime = std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
        .expect("fake eslint runtime should be readable")
        .replace(
            r#"mdxContentGlobs: ["src/content/**/*.mdx"], approvedMdxComponentModules: ["src/components/mdx/**/*"]"#,
            r#"mdxContentGlobs: ["src/pages/**/*.mdx"], approvedMdxComponentModules: ["src/components/mdx/**/*"]"#,
        );
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        eslint_runtime,
    )
    .expect("fake eslint runtime should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/content/posts/example.mdx",
            "src/components/mdx/index.tsx",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot
                    .mdx_content_effective_mdx_component_map_rules
                    .is_empty(),
                "wrong mdxContentGlobs must make MDX component-map rule ineffective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_adapter_source_without_runtime_astro_content_import() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("src/lib/content/index.ts"),
        "export const getContent = () => null;\n",
    )
    .expect("adapter source should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/lib/content/index.ts",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = &input.integration_contracts[0];

    assert_eq!(
        integration.approved_surface_sources.content_adapter,
        vec!["src/lib/content/index.ts".to_owned()],
        "adapter source should still exist: {integration:?}"
    );
    assert!(
        integration
            .approved_surface_sources
            .content_adapter_astro_content
            .is_empty(),
        "adapter source without runtime astro:content import must not be accepted: {integration:?}"
    );
}

#[test]
fn config_ingestion_rejects_adapter_source_with_type_only_astro_content_import() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("src/lib/content/index.ts"),
        "import type { CollectionEntry } from \"astro:content\";\nexport type Entry = CollectionEntry<\"posts\">;\n",
    )
    .expect("adapter source should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/lib/content/index.ts",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    assert!(
        input.integration_contracts[0]
            .approved_surface_sources
            .content_adapter_astro_content
            .is_empty(),
        "type-only astro:content import must not satisfy adapter runtime contract: {input:?}"
    );
}

#[test]
fn config_ingestion_accepts_syncpack_astro_policy_groups_as_a_set() {
    let root = super::helpers::fake_astro_workspace();
    let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
        .expect("root syncpack config should be readable")
        .replace(
            r#"    { "dependencies": ["react"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.5" },"#,
            r#"    { "dependencies": ["@project/app-specific"], "dependencyTypes": ["prod", "dev"], "pinVersion": "1.0.0" },
    { "dependencies": ["react"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.5" },"#,
        )
        .replace(
            r#"    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },"#,
            r#"    { "dependencies": ["@project/after-policy"], "dependencyTypes": ["prod", "dev"], "pinVersion": "2.0.0" },
    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },"#,
        );
    std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
        .expect("syncpack config should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "src/pages/card.tsx",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.integration_contracts[0].syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
            assert!(
                snapshot.missing_required_stack_pins.is_empty(),
                "extra app pins must not make required Astro pins look missing: {snapshot:?}"
            );
            assert!(
                snapshot.missing_forbidden_bans.is_empty(),
                "extra app pins must not make forbidden Astro bans look missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_accepts_syncpack_dependency_types_as_sets() {
    let root = super::helpers::fake_astro_workspace();
    let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
        .expect("root syncpack config should be readable")
        .replace(
            r#"    { "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" },"#,
            r#"    { "dependencies": ["astro"], "dependencyTypes": ["dev", "prod"], "pinVersion": "6.1.9" },"#,
        )
        .replace(
            r#"    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },"#,
            r#"    { "dependencies": ["next"], "dependencyTypes": ["peer", "optional", "dev", "prod"], "isBanned": true },"#,
        );
    std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
        .expect("syncpack config should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "eslint.config.mjs",
            "src/pages/index.astro",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.integration_contracts[0].syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
            assert!(
                snapshot.missing_required_stack_pins.is_empty(),
                "dependencyTypes order must not make required Astro pins look missing: {snapshot:?}"
            );
            assert!(
                snapshot.missing_forbidden_bans.is_empty(),
                "dependencyTypes order must not make forbidden Astro bans look missing: {snapshot:?}"
            );
        }
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_requires_all_contentlayer_syncpack_bans_from_real_config() {
    for dependency in [
        "contentlayer",
        "next-contentlayer",
        "@contentlayer/core",
        "@contentlayer/source-files",
    ] {
        let root = super::helpers::fake_astro_workspace();
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .lines()
            .filter(|line| !line.contains(&format!(r#""{dependency}""#)))
            .collect::<Vec<_>>()
            .join("\n")
            .replace(",\n  ]", "\n  ]");
        std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
            .expect("syncpack config should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "eslint.config.mjs",
                "src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.integration_contracts[0].syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => {
                assert!(
                    snapshot
                        .missing_forbidden_bans
                        .iter()
                        .any(|missing| missing == dependency),
                    "missing `{dependency}` ban should be detected: {snapshot:?}"
                );
            }
            other => panic!("expected parsed syncpack state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_requires_inline_public_content_rule_to_scan_copy_attributes() {
    for (case_name, original_policy, replacement_policy) in [
        (
            "exact copy attr exclude",
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden", "alt"] }"#,
        ),
        (
            "regex copy attr exclude",
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden", "aria-.+"] }"#,
        ),
        (
            "catch-all attr exclude",
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden", ".*"] }"#,
        ),
        (
            "attr include allowlist",
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
            r#""jsx-attributes": { include: ["className"], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
        ),
        (
            "non-string attr exclude",
            r#""jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }"#,
            r#""jsx-attributes": { include: [], exclude: [{}] }"#,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        let eslint_runtime =
            std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
                .expect("fake eslint runtime should be readable")
                .replace(original_policy, replacement_policy);
        std::fs::write(
            root.path().join("node_modules/eslint/index.js"),
            eslint_runtime,
        )
        .expect("fake eslint runtime should be rewritten");

        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
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
                        .ts_source_error_rules
                        .iter()
                        .any(|rule| rule == "i18next/no-literal-string"),
                    "inline public content rule should still be present at error severity for {case_name}: {snapshot:?}"
                );
                assert!(
                    snapshot
                        .ts_source_effective_inline_public_content_rules
                        .is_empty(),
                    "{case_name} must make delegated inline-copy policy ineffective: {snapshot:?}"
                );
            }
            other => panic!("expected parsed eslint state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_rejects_broad_inline_public_content_option_allowlists() {
    for (case_name, original_policy, replacement_policy) in [
        (
            "broad words exclude",
            r#"words: { include: [], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }"#,
            r#"words: { exclude: [".*"] }"#,
        ),
        (
            "js-only broad words exclude",
            r#"words: { include: [], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }"#,
            r#"words: { exclude: ["(?=.*Request an audit).*"] }"#,
        ),
        (
            "non-string words exclude",
            r#"words: { include: [], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }"#,
            r#"words: { exclude: [{}] }"#,
        ),
        (
            "words include allowlist",
            r#"words: { include: [], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }"#,
            r#"words: { include: ["OK"], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }"#,
        ),
        (
            "broad component exclude",
            r#""jsx-components": { include: [], exclude: [] }"#,
            r#""jsx-components": { include: [], exclude: [".*"] }"#,
        ),
        (
            "component include allowlist",
            r#""jsx-components": { include: [], exclude: [] }"#,
            r#""jsx-components": { include: ["CopyProbe"], exclude: [] }"#,
        ),
        (
            "non-string component exclude",
            r#""jsx-components": { include: [], exclude: [] }"#,
            r#""jsx-components": { include: [], exclude: [{}] }"#,
        ),
        (
            "object property name exclude",
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+"] }"#,
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+", "name"] }"#,
        ),
        (
            "object property title exclude",
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+"] }"#,
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+", "title"] }"#,
        ),
        (
            "object property include allowlist",
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+"] }"#,
            r#""object-properties": { include: ["id"], exclude: ["[A-Z_-]+"] }"#,
        ),
        (
            "non-string object property exclude",
            r#""object-properties": { include: [], exclude: ["[A-Z_-]+"] }"#,
            r#""object-properties": { include: [], exclude: [{}] }"#,
        ),
        (
            "i18n callee exclude",
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL", "i18n(ext)?"] }"#,
        ),
        (
            "postMessage callee exclude",
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL", "postMessage"] }"#,
        ),
        (
            "member callee exclude",
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL", "z\\.enum"] }"#,
        ),
        (
            "callee include allowlist",
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
            r#"callees: { include: ["clsx"], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
        ),
        (
            "non-string callee exclude",
            r#"callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }"#,
            r#"callees: { include: [], exclude: [{}] }"#,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        let eslint_runtime =
            std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
                .expect("fake eslint runtime should be readable")
                .replace(original_policy, replacement_policy);
        std::fs::write(
            root.path().join("node_modules/eslint/index.js"),
            eslint_runtime,
        )
        .expect("fake eslint runtime should be rewritten");

        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "eslint.config.mjs",
                "src/pages/index.astro",
                "src/pages/index.ts",
                "node_modules/eslint/index.js",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.eslint_contracts[0].config {
            G3TsAstroEslintSurfaceState::Parsed { snapshot } => assert!(
                snapshot
                    .ts_source_effective_inline_public_content_rules
                    .is_empty(),
                "{case_name} must make delegated inline-copy policy ineffective: {snapshot:?}"
            ),
            other => panic!("expected parsed eslint state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_sets_syncpack_lint_safety_from_real_package_scripts() {
    for (case_name, scripts_json, expected_safe) in [
        (
            "canonical",
            r#"{ "check": "astro check && syncpack lint" }"#,
            true,
        ),
        (
            "fake text",
            r#"{ "check": "astro check && echo syncpack lint" }"#,
            false,
        ),
        (
            "leading fail-open",
            r#"{ "check": "true || syncpack lint" }"#,
            false,
        ),
        (
            "later fail-open",
            r#"{ "check": "syncpack lint && true || true" }"#,
            false,
        ),
        (
            "duplicate unsafe surface",
            r#"{ "check": "astro check && syncpack lint", "test": "syncpack lint || true" }"#,
            false,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        std::fs::write(
            root.path().join("package.json"),
            format!(
                "{{\n  \"devDependencies\": {{\n    \"astro\": \"1.0.0\",\n    \"syncpack\": \"1.0.0\"\n  }},\n  \"scripts\": {scripts_json}\n}}\n"
            ),
        )
        .expect("package manifest should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.integration_contracts[0].package {
            G3TsAstroPackageSurfaceState::Parsed { snapshot } => assert_eq!(
                snapshot.safely_runs_syncpack_lint, expected_safe,
                "case {case_name} had wrong syncpack lint safety fact: {snapshot:?}"
            ),
            other => panic!("expected parsed package state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_requires_safe_astro_build_in_build_script() {
    for (case_name, scripts_json, expected_safe) in [
        (
            "canonical build script",
            r#"{ "check": "astro check", "build": "astro build" }"#,
            true,
        ),
        (
            "astro build hidden in check script",
            r#"{ "check": "astro build" }"#,
            false,
        ),
        (
            "fail open build script",
            r#"{ "build": "astro build || true" }"#,
            false,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        std::fs::write(
            root.path().join("package.json"),
            format!(
                "{{\n  \"devDependencies\": {{\n    \"astro\": \"1.0.0\"\n  }},\n  \"scripts\": {scripts_json}\n}}\n"
            ),
        )
        .expect("package manifest should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.integration_contracts[0].package {
            G3TsAstroPackageSurfaceState::Parsed { snapshot } => assert_eq!(
                snapshot.safely_runs_astro_build, expected_safe,
                "case {case_name} had wrong astro build safety fact: {snapshot:?}"
            ),
            other => panic!("expected parsed package state, got {other:?}"),
        }
    }
}

#[test]
fn app_local_syncpack_rejects_repo_relative_source_entry() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::create_dir_all(root.path().join("apps/landing/src/pages"))
        .expect("nested pages directory should be created");
    std::fs::create_dir_all(root.path().join("apps/landing/src/content"))
        .expect("nested content directory should be created");
    std::fs::write(
        root.path().join("apps/landing/package.json"),
        "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\",\n    \"syncpack\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check && syncpack lint\"\n  }\n}\n",
    )
    .expect("nested package manifest should be written");
    std::fs::write(
        root.path().join("apps/landing/astro.config.mjs"),
        "export default {};\n",
    )
    .expect("nested astro config should be written");
    std::fs::write(
        root.path().join("apps/landing/src/content.config.ts"),
        "export default {};\n",
    )
    .expect("nested content config should be written");
    let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
        .expect("root syncpack config should be readable")
        .replace(
            "\"source\": [\"package.json\"]",
            "\"source\": [\"apps/landing/package.json\"]",
        );
    std::fs::write(
        root.path().join("apps/landing/.syncpackrc"),
        syncpack_config,
    )
    .expect("nested syncpack config should be written");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "apps/landing/package.json",
            "apps/landing/astro.config.mjs",
            "apps/landing/src/content.config.ts",
            "apps/landing/.syncpackrc",
            "apps/landing/src/pages/index.astro",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = input
        .integration_contracts
        .iter()
        .find(|contract| contract.app_root_rel_path == "apps/landing")
        .expect("nested Astro app contract should exist");

    match &integration.syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
            !snapshot.source_covers_package_manifest,
            "app-local .syncpackrc must not accept repo-relative source entries: {snapshot:?}"
        ),
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }
}

#[test]
fn root_syncpack_rejects_non_exact_sources_for_nested_app_manifest() {
    for source_entry in ["package.json", "apps/*/package.json"] {
        let root = super::helpers::fake_astro_workspace();
        std::fs::create_dir_all(root.path().join("apps/landing/src/pages"))
            .expect("nested pages directory should be created");
        std::fs::create_dir_all(root.path().join("apps/landing/src/content"))
            .expect("nested content directory should be created");
        std::fs::write(
            root.path().join("apps/landing/package.json"),
            "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\",\n    \"syncpack\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check && syncpack lint\"\n  }\n}\n",
        )
        .expect("nested package manifest should be written");
        std::fs::write(
            root.path().join("apps/landing/astro.config.mjs"),
            "export default {};\n",
        )
        .expect("nested astro config should be written");
        std::fs::write(
            root.path().join("apps/landing/src/content.config.ts"),
            "export default {};\n",
        )
        .expect("nested content config should be written");
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .replace(
                "\"source\": [\"package.json\"]",
                &format!("\"source\": [\"{source_entry}\"]"),
            );
        std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
            .expect("root syncpack config should be rewritten");

        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "apps/landing/package.json",
                "apps/landing/astro.config.mjs",
                "apps/landing/src/content.config.ts",
                ".syncpackrc",
                "apps/landing/src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        let integration = input
            .integration_contracts
            .iter()
            .find(|contract| contract.app_root_rel_path == "apps/landing")
            .expect("nested Astro app contract should exist");

        match &integration.syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Missing { rel_path } => {
                assert_eq!(rel_path, "apps/landing/.syncpackrc");
            }
            other => panic!("expected missing app-local syncpack state, got {other:?}"),
        }
    }
}

#[test]
fn root_syncpack_rejects_source_alias_entries() {
    for source_entry in [
        "./package.json",
        "/package.json",
        "foo/../package.json",
        "../package.json",
    ] {
        let root = super::helpers::fake_astro_workspace();
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .replace(
                "\"source\": [\"package.json\"]",
                &format!("\"source\": [\"{source_entry}\"]"),
            );
        std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
            .expect("syncpack config should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        let integration = &input.integration_contracts[0];

        match &integration.syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
                !snapshot.source_covers_package_manifest,
                "root .syncpackrc must not accept alias source entry {source_entry:?}: {snapshot:?}"
            ),
            other => panic!("expected parsed syncpack state, got {other:?}"),
        }
    }
}

#[test]
fn syncpack_source_must_be_exactly_the_app_manifest() {
    let root = super::helpers::fake_astro_workspace();
    let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
        .expect("root syncpack config should be readable")
        .replace(
            "\"source\": [\"package.json\"]",
            "\"source\": [\"package.json\", \"other/package.json\"]",
        );
    std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
        .expect("syncpack config should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "src/pages/index.html",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = &input.integration_contracts[0];

    match &integration.syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
            !snapshot.source_covers_package_manifest,
            "source must contain only package.json for this app: {snapshot:?}"
        ),
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }
}

#[test]
fn app_local_syncpack_rejects_source_alias_entries() {
    for source_entry in [
        "./package.json",
        "/package.json",
        "foo/../package.json",
        "../landing/package.json",
    ] {
        let root = super::helpers::fake_astro_workspace();
        std::fs::create_dir_all(root.path().join("apps/landing/src/pages"))
            .expect("nested pages directory should be created");
        std::fs::create_dir_all(root.path().join("apps/landing/src/content"))
            .expect("nested content directory should be created");
        std::fs::write(
            root.path().join("apps/landing/package.json"),
            "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\",\n    \"syncpack\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check && syncpack lint\"\n  }\n}\n",
        )
        .expect("nested package manifest should be written");
        std::fs::write(
            root.path().join("apps/landing/astro.config.mjs"),
            "export default {};\n",
        )
        .expect("nested astro config should be written");
        std::fs::write(
            root.path().join("apps/landing/src/content.config.ts"),
            "export default {};\n",
        )
        .expect("nested content config should be written");
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .replace(
                "\"source\": [\"package.json\"]",
                &format!("\"source\": [\"{source_entry}\"]"),
            );
        std::fs::write(
            root.path().join("apps/landing/.syncpackrc"),
            syncpack_config,
        )
        .expect("nested syncpack config should be written");

        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "apps/landing/package.json",
                "apps/landing/astro.config.mjs",
                "apps/landing/src/content.config.ts",
                "apps/landing/.syncpackrc",
                "apps/landing/src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        let integration = input
            .integration_contracts
            .iter()
            .find(|contract| contract.app_root_rel_path == "apps/landing")
            .expect("nested Astro app contract should exist");

        match &integration.syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
                !snapshot.source_covers_package_manifest,
                "app-local .syncpackrc must not accept alias source entry {source_entry:?}: {snapshot:?}"
            ),
            other => panic!("expected parsed syncpack state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_rejects_noncanonical_required_pin_groups() {
    let canonical_astro_pin = r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" }"#;

    for (case_name, replacement) in [
        (
            "shadowed",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" },
    { "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" }"#,
        ),
        (
            "shadowed with wrong dependencyTypes",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" },
    { "dependencies": ["astro"], "dependencyTypes": ["prod"], "pinVersion": "6.1.9" }"#,
        ),
        (
            "wrong dependencyTypes",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod"], "pinVersion": "6.1.9" }"#,
        ),
        (
            "wrong version",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.8" }"#,
        ),
        (
            "package scoped",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "packages": ["other-package"], "pinVersion": "6.1.9" }"#,
        ),
        (
            "specifier scoped",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "specifierTypes": ["!exact"], "pinVersion": "6.1.9" }"#,
        ),
        (
            "ignored",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9", "isIgnored": true }"#,
        ),
        (
            "banned",
            r#"{ "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9", "isBanned": true }"#,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .replace(canonical_astro_pin, replacement);
        std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
            .expect("syncpack config should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.integration_contracts[0].syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
                snapshot
                    .missing_required_stack_pins
                    .iter()
                    .any(|pin| pin.dependency == "astro"),
                "case {case_name} should make astro pin missing: {snapshot:?}"
            ),
            other => panic!("expected parsed syncpack state for {case_name}, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_rejects_noncanonical_forbidden_ban_groups() {
    let canonical_next_ban = r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true }"#;

    for (case_name, replacement) in [
        (
            "shadowed",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": false },
    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true }"#,
        ),
        (
            "shadowed with wrong dependencyTypes",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev"], "isBanned": true }"#,
        ),
        (
            "package scoped",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "packages": ["other-package"], "isBanned": true }"#,
        ),
        (
            "specifier scoped",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "specifierTypes": ["!exact"], "isBanned": true }"#,
        ),
        (
            "wrong dependencyTypes",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev"], "isBanned": true }"#,
        ),
        (
            "ignored",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true, "isIgnored": true }"#,
        ),
        (
            "pinVersion present",
            r#"{ "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true, "pinVersion": "0.0.0" }"#,
        ),
    ] {
        let root = super::helpers::fake_astro_workspace();
        let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
            .expect("root syncpack config should be readable")
            .replace(canonical_next_ban, replacement);
        std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
            .expect("syncpack config should be rewritten");
        let crawl = super::helpers::crawl_with_entries(
            &root,
            &[
                "package.json",
                "astro.config.mjs",
                "src/content.config.ts",
                ".syncpackrc",
                "src/pages/index.astro",
            ],
        );

        let input = super::super::ingest_for_config_checks(&crawl);
        match &input.integration_contracts[0].syncpack_config {
            g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
                snapshot
                    .missing_forbidden_bans
                    .iter()
                    .any(|dependency| dependency == "next"),
                "case {case_name} should report next as missing: {snapshot:?}"
            ),
            other => panic!("expected parsed syncpack state, got {other:?}"),
        }
    }
}

#[test]
fn config_ingestion_requires_astro_seo_forbidden_ban() {
    let root = super::helpers::fake_astro_workspace();
    let astro_seo_ban = r#"    { "dependencies": ["astro-seo"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
"#;
    let syncpack_config = std::fs::read_to_string(root.path().join(".syncpackrc"))
        .expect("root syncpack config should be readable")
        .replace(astro_seo_ban, "");
    std::fs::write(root.path().join(".syncpackrc"), syncpack_config)
        .expect("syncpack config should be rewritten");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "src/pages/index.astro",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.integration_contracts[0].syncpack_config {
        g3ts_astro_types::G3TsAstroSyncpackConfigState::Parsed { snapshot } => assert!(
            snapshot
                .missing_forbidden_bans
                .iter()
                .any(|dependency| dependency == "astro-seo"),
            "missing astro-seo ban should be reported: {snapshot:?}"
        ),
        other => panic!("expected parsed syncpack state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_route_scoped_pipeline_rules_when_no_page_routes_exist() {
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-velite-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
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
            "src/pages/feed.json.ts",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert_eq!(
                snapshot
                    .ts_source_effective_route_scoped_pipeline_rules
                    .len(),
                0,
                "route-scoped rules require at least one actual page route: {snapshot:?}"
            );
            assert_eq!(
                snapshot
                    .ts_source_effective_content_data_pipeline_rules
                    .len(),
                0,
                "content-data rule requires route coverage before it is effective: {snapshot:?}"
            );
            assert_eq!(
                snapshot
                    .ts_source_effective_content_source_pipeline_rules
                    .len(),
                0,
                "content-source rules require route coverage before they are effective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_endpoint_only_scope_options_when_route_pages_exist() {
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-velite-imports": ["error", { endpointGlobs: ["src/pages/**/*.json.ts"] }],
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
            "src/pages/feed.json.ts",
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
                "endpoint-only coverage must not count when route pages exist: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_invalid_endpoint_scope_options_even_when_no_endpoints_exist() {
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["["] }],
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
                "invalid endpoint globs must not count as effective with an empty endpoint set: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_partial_route_coverage_for_route_scoped_rules() {
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: ["src/pages/index.astro"], endpointGlobs: ["src/pages/**/*.json.ts"] }],
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
            "src/pages/blog.astro",
            "src/pages/feed.json.ts",
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
                "partial route coverage must not count as effective: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_data_pipeline_rules
                    .is_empty(),
                "content-data rules must not count as effective when route coverage is partial: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_source_pipeline_rules
                    .is_empty(),
                "content-source rules must not count as effective when route coverage is partial: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_rejects_partial_endpoint_coverage_for_route_scoped_rules() {
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: ["src/pages/**/*.astro"], endpointGlobs: ["src/pages/feed.json.ts"] }],
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
            "src/pages/feed.json.ts",
            "src/pages/search.json.ts",
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
                "partial endpoint coverage must not count as effective: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_data_pipeline_rules
                    .is_empty(),
                "content-data rules must not count as effective when endpoint coverage is partial: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_source_pipeline_rules
                    .is_empty(),
                "content-source rules must not count as effective when endpoint coverage is partial: {snapshot:?}"
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
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: [1], authoredContentGlobs: [1] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: [1], authoredContentGlobs: [1] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: [1], authoredContentGlobs: [1] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: [1], contentDataModuleGlobs: [1] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: [1] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: [1] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: [1] }],
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
            assert!(
                snapshot
                    .ts_source_effective_content_data_pipeline_rules
                    .is_empty(),
                "malformed content-data scope options must not count as effective: {snapshot:?}"
            );
            assert!(
                snapshot
                    .ts_source_effective_content_source_pipeline_rules
                    .is_empty(),
                "malformed content-source scope options must not count as effective: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_uses_only_first_eslint_rule_options_object_for_pipeline_scope() {
    let root = super::helpers::fake_astro_workspace();
    let eslint_runtime = std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
        .expect("fake eslint runtime should be readable")
        .replace(
            r#""astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }]"#,
            r#""astro-pipeline/no-content-data-modules-in-routes": ["error", {}, { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }]"#,
        );
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        eslint_runtime,
    )
    .expect("fake eslint runtime should be rewritten");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "src/pages/card.tsx",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => assert!(
            !snapshot
                .ts_source_effective_content_data_pipeline_rules
                .iter()
                .any(|rule| rule == "astro-pipeline/no-content-data-modules-in-routes"),
            "valid options in later objects must not count as effective: {snapshot:?}"
        ),
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn config_ingestion_does_not_mark_warn_pipeline_rules_effective() {
    let root = super::helpers::fake_astro_workspace();
    let eslint_runtime = std::fs::read_to_string(root.path().join("node_modules/eslint/index.js"))
        .expect("fake eslint runtime should be readable")
        .replace(
            r#""astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }]"#,
            r#""astro-pipeline/no-content-data-modules-in-routes": ["warn", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }]"#,
        );
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        eslint_runtime,
    )
    .expect("fake eslint runtime should be rewritten");

    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/pages/index.ts",
            "src/pages/card.tsx",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);

    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => assert!(
            !snapshot
                .ts_source_effective_content_data_pipeline_rules
                .iter()
                .any(|rule| rule == "astro-pipeline/no-content-data-modules-in-routes"),
            "warn severity must not count as effective: {snapshot:?}"
        ),
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn filetree_ingestion_discovers_nested_velite_surfaces_under_astro_root() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "src/content/landing/velite.config.ts",
            "src/generated/.velite/landing.js",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert_eq!(
        app_root.velite_config_rel_path.as_deref(),
        Some("src/content/landing/velite.config.ts")
    );
    assert_eq!(
        app_root.velite_output_rel_paths,
        vec!["src/generated/.velite/landing.js".to_owned()]
    );
}

#[test]
fn filetree_ingestion_discovers_legacy_generated_state_under_astro_root() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".next/server/app/page.js",
            ".contentlayer/generated/Post.mjs",
            "contentlayer.config.ts",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert_eq!(
        app_root.legacy_generated_state_rel_paths,
        vec![
            ".next/server/app/page.js".to_owned(),
            ".contentlayer/generated/Post.mjs".to_owned(),
            "contentlayer.config.ts".to_owned(),
        ]
    );
}

#[test]
fn filetree_ingestion_discovers_ignored_legacy_generated_state_sentinels() {
    let root = super::helpers::fake_astro_workspace();
    let mut crawl = super::helpers::crawl_with_entries(
        &root,
        &["package.json", "astro.config.mjs", "src/content.config.ts"],
    );
    crawl.entries.extend([
        super::helpers::ignored_directory_entry(&root, ".next"),
        super::helpers::ignored_directory_entry(&root, ".contentlayer"),
        super::helpers::ignored_entry(&root, "contentlayer.config.ts"),
    ]);

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert_eq!(
        app_root.legacy_generated_state_rel_paths,
        vec![
            ".next".to_owned(),
            ".contentlayer".to_owned(),
            "contentlayer.config.ts".to_owned(),
        ]
    );
}

#[test]
fn filetree_ingestion_discovers_gitignored_generated_state_from_real_crawl() {
    let root = super::helpers::fake_astro_workspace();
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root.path())
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init failed with {status}");
    std::fs::write(
        root.path().join(".gitignore"),
        ".next/\n.velite/\n.contentlayer/\n",
    )
    .expect("gitignore should be written");
    std::fs::create_dir_all(root.path().join(".next/server/app"))
        .expect(".next output should be created");
    std::fs::create_dir_all(root.path().join(".velite")).expect(".velite output should be created");
    std::fs::create_dir_all(root.path().join(".contentlayer/generated"))
        .expect(".contentlayer output should be created");

    let crawl = g3_workspace_crawl::crawl(root.path()).expect("real crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert!(
        app_root
            .legacy_generated_state_rel_paths
            .iter()
            .any(|path| path == ".next"),
        "real crawl should recover ignored .next sentinel: {app_root:?}"
    );
    assert!(
        app_root
            .legacy_generated_state_rel_paths
            .iter()
            .any(|path| path == ".contentlayer"),
        "real crawl should recover ignored .contentlayer sentinel: {app_root:?}"
    );
    assert!(
        app_root
            .velite_output_rel_paths
            .iter()
            .any(|path| path == ".velite"),
        "real crawl should recover ignored .velite sentinel: {app_root:?}"
    );
}

#[test]
fn filetree_ingestion_does_not_misclassify_contentlayer_config_route_files() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "src/pages/contentlayer.config.ts",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert!(
        app_root.legacy_generated_state_rel_paths.is_empty(),
        "route files named contentlayer.config.* must not be treated as root Contentlayer config: {app_root:?}"
    );
}

#[test]
fn filetree_ingestion_assigns_legacy_state_to_nearest_nested_astro_root() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "apps/child/package.json",
            "apps/child/astro.config.mjs",
            "apps/child/src/content.config.ts",
            "apps/child/.next/server/app/page.js",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let parent = input
        .app_roots
        .iter()
        .find(|root| root.app_root_rel_path == ".")
        .expect("parent Astro root should be discovered");
    let child = input
        .app_roots
        .iter()
        .find(|root| root.app_root_rel_path == "apps/child")
        .expect("child Astro root should be discovered");

    assert!(
        parent.legacy_generated_state_rel_paths.is_empty(),
        "parent must not own nested child generated state: {parent:?}"
    );
    assert_eq!(
        child.legacy_generated_state_rel_paths,
        vec!["apps/child/.next/server/app/page.js".to_owned()]
    );
}

#[test]
fn filetree_ingestion_discovers_empty_included_legacy_state_directories() {
    let root = super::helpers::fake_astro_workspace();
    let mut crawl = super::helpers::crawl_with_entries(
        &root,
        &["package.json", "astro.config.mjs", "src/content.config.ts"],
    );
    crawl.entries.extend([
        super::helpers::included_directory_entry(&root, ".next"),
        super::helpers::included_directory_entry(&root, ".contentlayer"),
    ]);

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert_eq!(
        app_root.legacy_generated_state_rel_paths,
        vec![".next".to_owned(), ".contentlayer".to_owned()]
    );
}

#[test]
fn filetree_ingestion_does_not_misclassify_route_files_named_velite_config() {
    let root = super::helpers::fake_astro_workspace();
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            "src/pages/velite.config.ts",
        ],
    );

    let input = super::super::ingest_for_file_tree_checks(&crawl);
    let app_root = input
        .app_roots
        .first()
        .expect("astro root should be discovered");

    assert_eq!(app_root.velite_config_rel_path, None);
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
    assert_eq!(eslint.app_root_rel_path, ".");
}

#[test]
fn config_ingestion_preserves_multiple_configured_content_adapters() {
    let root = super::helpers::fake_astro_workspace();
    let guardrail_config = std::fs::read_to_string(root.path().join("guardrail3-ts.toml"))
        .expect("guardrail config should be readable")
        .replace(
            r#"adapters = ["src/lib/content"]"#,
            r#"adapters = ["src/lib/content", "src/lib/secondary-content"]"#,
        );
    std::fs::write(root.path().join("guardrail3-ts.toml"), guardrail_config)
        .expect("guardrail config should be rewritten");
    std::fs::create_dir_all(root.path().join("src/lib/secondary-content"))
        .expect("secondary adapter directory should be created");
    std::fs::write(
        root.path().join("src/lib/secondary-content/index.ts"),
        "import { getCollection } from \"astro:content\";\nexport const getSecondaryContent = () => getCollection;\n",
    )
    .expect("secondary adapter source should be written");
    let crawl = super::helpers::crawl_with_entries(
        &root,
        &[
            "package.json",
            "astro.config.mjs",
            "src/content.config.ts",
            ".syncpackrc",
            "guardrail3-ts.toml",
            "eslint.config.mjs",
            "src/pages/index.astro",
            "src/lib/content/index.ts",
            "src/lib/secondary-content/index.ts",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    let integration = &input.integration_contracts[0];
    let G3TsAstroPolicySurfaceState::Parsed { snapshot } = &integration.astro_policy else {
        panic!("astro policy should be parsed");
    };

    assert_eq!(
        snapshot.content_adapters,
        vec![
            "src/lib/content".to_owned(),
            "src/lib/secondary-content".to_owned(),
        ],
        "all configured content adapters should be preserved: {snapshot:?}"
    );
    assert_eq!(
        integration.approved_surface_sources.content_adapter,
        vec![
            "src/lib/content/index.ts".to_owned(),
            "src/lib/secondary-content/index.ts".to_owned(),
        ],
        "all configured content adapter source paths should be discovered: {integration:?}"
    );
    assert_eq!(
        integration
            .approved_surface_sources
            .content_adapter_astro_content,
        vec![
            "src/lib/content/index.ts".to_owned(),
            "src/lib/secondary-content/index.ts".to_owned(),
        ],
        "all configured content adapters should be checked for runtime astro:content imports: {integration:?}"
    );
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

    assert_eq!(
        input.integration_contracts.len(),
        1,
        "unexpected inputs: {input:?}"
    );
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

    assert_eq!(
        input.app_roots.len(),
        1,
        "unexpected file-tree inputs: {input:?}"
    );
    assert_eq!(
        input.build_collection_roots.len(),
        1,
        "unexpected build-collection roots: {input:?}"
    );
    assert_eq!(
        input.route_markdown_pages.len(),
        1,
        "unexpected markdown pages: {input:?}"
    );
    assert_eq!(
        input.route_markdown_pages[0].rel_path, "src/pages/about.mdx",
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
    assert_eq!(
        config_input.integration_contracts.len(),
        1,
        "unexpected config inputs: {config_input:?}"
    );
    assert_eq!(
        config_input.eslint_contracts.len(),
        1,
        "unexpected config inputs: {config_input:?}"
    );

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
    assert_eq!(
        file_tree_input.app_roots.len(),
        1,
        "unexpected file-tree inputs: {file_tree_input:?}"
    );
    assert_eq!(
        file_tree_input.app_roots[0].app_root_rel_path,
        "apps/landing"
    );
    assert_eq!(
        file_tree_input.app_roots[0]
            .astro_config_rel_path
            .as_deref(),
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
            assert!(
                !snapshot.astro_source_probe_present,
                "ignored astro probe should not count: {snapshot:?}"
            );
            assert!(
                snapshot.ts_source_probe_present,
                "ts lane should stay present: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn source_lane_probes_use_only_src_files_or_exact_synthetic_fallbacks() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::create_dir_all(root.path().join("scripts")).expect("scripts dir should be created");
    std::fs::create_dir_all(root.path().join("components"))
        .expect("components dir should be created");
    std::fs::create_dir_all(root.path().join("tools")).expect("tools dir should be created");
    std::fs::write(root.path().join("scripts/build.ts"), "export {};\n")
        .expect("script ts file should be written");
    std::fs::write(
        root.path().join("components/card.astro"),
        "---\n---\n<div />\n",
    )
    .expect("non-src astro file should be written");
    std::fs::write(
        root.path().join("tools/card.tsx"),
        "export function Card() { return null; }\n",
    )
    .expect("non-src tsx file should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(filePath) {
    return String(filePath).endsWith("src/__g3ts_probe__.astro")
      || String(filePath).endsWith("src/index.ts")
      || String(filePath).endsWith("src/__g3ts_probe__.tsx");
  }

  async calculateConfigForFile(_filePath) {
    return { plugins: { astro: {}, "astro-pipeline": {} }, rules: {} };
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
            "components/card.astro",
            "scripts/build.ts",
            "tools/card.tsx",
            "src/pages/index.html",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                !snapshot.astro_source_probe_present,
                "non-src astro files must not be selected as AstroSource probes: {snapshot:?}"
            );
            assert!(
                snapshot.ts_source_probe_present,
                "src/content.config.ts satisfies the written TsSource probe glob: {snapshot:?}"
            );
            assert!(
                !snapshot.tsx_source_probe_present,
                "non-src tsx files must not be selected as TsxSource probes: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}

#[test]
fn source_lane_probes_follow_written_src_globs_without_test_or_config_exclusions() {
    let root = super::helpers::fake_astro_workspace();
    std::fs::write(
        root.path().join("src/vitest.config.ts"),
        "export default {};\n",
    )
    .expect("src config ts file should be written");
    std::fs::write(
        root.path().join("src/component.spec.tsx"),
        "export function Component() { return null; }\n",
    )
    .expect("src spec tsx file should be written");
    std::fs::write(
        root.path().join("src/layout.config.astro"),
        "---\n---\n<div />\n",
    )
    .expect("src config astro file should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(filePath) {
    return String(filePath).endsWith("src/__g3ts_probe__.astro")
      || String(filePath).endsWith("src/index.ts")
      || String(filePath).endsWith("src/__g3ts_probe__.tsx");
  }

  async calculateConfigForFile(_filePath) {
    return { plugins: { astro: {}, "astro-pipeline": {} }, rules: {} };
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
            "src/layout.config.astro",
            "src/vitest.config.ts",
            "src/component.spec.tsx",
            "src/pages/index.html",
            "node_modules/eslint/index.js",
        ],
    );

    let input = super::super::ingest_for_config_checks(&crawl);
    match &input.eslint_contracts[0].config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => {
            assert!(
                snapshot.astro_source_probe_present,
                "src/**/*.astro config-like files still satisfy the written AstroSource probe glob: {snapshot:?}"
            );
            assert!(
                snapshot.ts_source_probe_present,
                "src/**/*.ts config-like files still satisfy the written TsSource probe glob: {snapshot:?}"
            );
            assert!(
                snapshot.tsx_source_probe_present,
                "src/**/*.tsx spec files still satisfy the written TsxSource probe glob: {snapshot:?}"
            );
        }
        other => panic!("expected parsed eslint state, got {other:?}"),
    }
}
