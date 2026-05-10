use g3ts_astro_content_types::{
    G3TsAstroContentAdapterRootInput, G3TsAstroContentAdapterSourceInput,
    G3TsAstroContentAdapterSourcePaths, G3TsAstroContentConfigChecksInput,
    G3TsAstroContentEslintPluginContractInput, G3TsAstroContentEslintSurfaceSnapshot,
    G3TsAstroContentEslintSurfaceState, G3TsAstroContentIntegrationContractInput,
    G3TsAstroContentPolicyEslintContractInput, G3TsAstroContentPolicySnapshot,
    G3TsAstroContentPolicySurfaceState, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroPipelineRuleScopeSnapshot,
};
use std::collections::BTreeMap;

/// Returns a mutable reference to the parsed eslint snapshot of the first eslint contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first eslint contract must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn eslint_snapshot_mut(
    input: &mut G3TsAstroContentConfigChecksInput,
) -> &mut G3TsAstroContentEslintSurfaceSnapshot {
    let config = &mut input.eslint_contracts[0].config;
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden content eslint config should be parsed");
    };
    snapshot
}

/// Returns a mutable reference to the parsed eslint snapshot of the first policy-eslint contract.
///
/// Panics when the golden fixture state has been altered to a non-parsed variant, which the
/// surrounding test treats as a setup failure. Test-only helper.
#[expect(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "golden fixture invariant: first policy_eslint contract must be Parsed; mismatch is a test setup failure"
)]
pub(super) fn policy_eslint_snapshot_mut(
    input: &mut G3TsAstroContentConfigChecksInput,
) -> &mut G3TsAstroContentEslintSurfaceSnapshot {
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden content policy-eslint config should be parsed");
    };
    snapshot
}

pub(super) fn golden() -> G3TsAstroContentConfigChecksInput {
    let astro_policy = astro_policy();
    let eslint_config = eslint_config();
    G3TsAstroContentConfigChecksInput {
        integration_contracts: vec![G3TsAstroContentIntegrationContractInput {
            app_root_rel_path: ".".to_owned(),
            route_page_paths: vec!["src/pages/index.astro".to_owned()],
            endpoint_paths: vec!["src/pages/rss.ts".to_owned()],
            content_adapter_sources: G3TsAstroContentAdapterSourcePaths {
                content_adapter: vec!["src/lib/content/index.ts".to_owned()],
                content_adapter_astro_content: vec!["src/lib/content/index.ts".to_owned()],
            },
            package: package(),
            astro_policy: astro_policy.clone(),
        }],
        eslint_contracts: vec![G3TsAstroContentEslintPluginContractInput {
            app_root_rel_path: ".".to_owned(),
            config: eslint_config.clone(),
        }],
        policy_eslint_contracts: vec![G3TsAstroContentPolicyEslintContractInput {
            app_root_rel_path: ".".to_owned(),
            route_page_paths: vec!["src/pages/index.astro".to_owned()],
            endpoint_paths: vec!["src/pages/rss.ts".to_owned()],
            astro_policy,
            eslint_config,
        }],
        eslint_directives: Vec::new(),
        adapter_root_contracts: vec![G3TsAstroContentAdapterRootInput {
            policy_rel_path: "guardrail3-ts.toml".to_owned(),
            configured_adapter: "src/lib/content".to_owned(),
            source_exists: true,
        }],
        adapter_source_contracts: vec![G3TsAstroContentAdapterSourceInput {
            policy_rel_path: "guardrail3-ts.toml".to_owned(),
            source_rel_path: "src/lib/content/index.ts".to_owned(),
            imports_astro_content: true,
        }],
    }
}

fn package() -> G3TsAstroPackageSurfaceState {
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            package_name: Some("landing".to_owned()),
            dependencies: Vec::new(),
            dev_dependencies: vec![
                "eslint-plugin-i18next".to_owned(),
                "g3ts-eslint-plugin-astro-pipeline".to_owned(),
            ],
            script_names: Vec::new(),
            script_bodies: Vec::new(),
            script_commands: Vec::new(),
            script_tool_invocations: Vec::new(),
            script_parse_blockers: Vec::new(),
        },
    }
}

fn astro_policy() -> G3TsAstroContentPolicySurfaceState {
    G3TsAstroContentPolicySurfaceState::Parsed {
        snapshot: G3TsAstroContentPolicySnapshot {
            rel_path: "guardrail3-ts.toml".to_owned(),
            profile: Some("strict-static-content".to_owned()),
            content_routes: vec!["src/pages/**/*.astro".to_owned()],
            non_content_routes: vec!["src/pages/404.astro".to_owned()],
            endpoints: vec!["src/pages/**/*.ts".to_owned()],
            content_root: Some("src/content".to_owned()),
            content_adapters: vec!["src/lib/content".to_owned()],
            required_collections: Vec::new(),
            collection_fields: BTreeMap::new(),
        },
    }
}

fn eslint_config() -> G3TsAstroContentEslintSurfaceState {
    let pipeline_rules = vec![
        "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
        "i18next/no-literal-string".to_owned(),
    ];
    let pipeline_scopes = vec![G3TsAstroPipelineRuleScopeSnapshot {
        rule_name: "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
        route_globs: vec!["src/pages/**/*.astro".to_owned()],
        endpoint_globs: vec!["src/pages/**/*.ts".to_owned()],
    }];
    G3TsAstroContentEslintSurfaceState::Parsed {
        snapshot: G3TsAstroContentEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            astro_source_probe_present: true,
            ts_source_probe_present: true,
            tsx_source_probe_present: true,
            astro_source_plugins: vec!["astro-pipeline".to_owned(), "i18next".to_owned()],
            ts_source_plugins: vec!["astro-pipeline".to_owned(), "i18next".to_owned()],
            tsx_source_plugins: vec!["astro-pipeline".to_owned(), "i18next".to_owned()],
            astro_source_error_rules: pipeline_rules.clone(),
            ts_source_error_rules: pipeline_rules.clone(),
            tsx_source_error_rules: pipeline_rules,
            astro_source_effective_content_adapter_modules: vec!["src/lib/content/**".to_owned()],
            ts_source_effective_content_adapter_modules: vec!["src/lib/content/**".to_owned()],
            tsx_source_effective_content_adapter_modules: vec!["src/lib/content/**".to_owned()],
            astro_source_route_scoped_pipeline_rule_scopes: pipeline_scopes.clone(),
            ts_source_route_scoped_pipeline_rule_scopes: pipeline_scopes.clone(),
            tsx_source_route_scoped_pipeline_rule_scopes: pipeline_scopes,
            astro_source_effective_inline_public_content_rules: vec![
                "i18next/no-literal-string".to_owned(),
            ],
            ts_source_effective_inline_public_content_rules: vec![
                "i18next/no-literal-string".to_owned(),
            ],
            tsx_source_effective_inline_public_content_rules: vec![
                "i18next/no-literal-string".to_owned(),
            ],
            astro_source_warn_or_error_rules: warn_or_error_rules(),
            ts_source_warn_or_error_rules: warn_or_error_rules(),
            tsx_source_warn_or_error_rules: warn_or_error_rules(),
            astro_source_restricted_disable_patterns: restricted_disable_patterns(),
            ts_source_restricted_disable_patterns: restricted_disable_patterns(),
            tsx_source_restricted_disable_patterns: restricted_disable_patterns(),
            astro_source_probe_ignored: false,
            ts_source_probe_ignored: false,
            tsx_source_probe_ignored: false,
        },
    }
}

fn warn_or_error_rules() -> Vec<String> {
    vec![
        "astro-pipeline/no-authored-content-fs-read".to_owned(),
        "astro-pipeline/no-authored-content-glob".to_owned(),
        "astro-pipeline/no-authored-content-imports".to_owned(),
        "astro-pipeline/no-content-data-modules-in-routes".to_owned(),
        "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
        "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
        "astro-pipeline/no-side-loader-imports".to_owned(),
        "astro-pipeline/no-velite-imports".to_owned(),
        "i18next/no-literal-string".to_owned(),
        "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
    ]
}

fn restricted_disable_patterns() -> Vec<String> {
    vec![
        "astro-pipeline/no-authored-content-fs-read".to_owned(),
        "astro-pipeline/no-authored-content-glob".to_owned(),
        "astro-pipeline/no-authored-content-imports".to_owned(),
        "astro-pipeline/no-content-data-modules-in-routes".to_owned(),
        "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
        "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
        "astro-pipeline/no-side-loader-imports".to_owned(),
        "astro-pipeline/no-velite-imports".to_owned(),
        "i18next/no-literal-string".to_owned(),
    ]
}
