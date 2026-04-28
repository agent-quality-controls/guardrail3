use g3ts_astro_mdx_types::{
    G3TsAstroMdxApprovedSourcePaths, G3TsAstroMdxConfigChecksInput,
    G3TsAstroMdxEslintPluginContractInput, G3TsAstroMdxEslintSurfaceSnapshot,
    G3TsAstroMdxEslintSurfaceState, G3TsAstroMdxIntegrationContractInput,
    G3TsAstroMdxPolicySnapshot, G3TsAstroMdxPolicySurfaceState, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState,
};
use std::collections::BTreeMap;

pub(super) fn golden() -> G3TsAstroMdxConfigChecksInput {
    G3TsAstroMdxConfigChecksInput {
        integration_contracts: vec![G3TsAstroMdxIntegrationContractInput {
            app_root_rel_path: ".".to_owned(),
            mdx_sources: G3TsAstroMdxApprovedSourcePaths {
                mdx_component_maps: vec!["src/mdx-components/index.tsx".to_owned()],
                missing_mdx_component_maps: Vec::new(),
            },
            package: package(),
            astro_policy: astro_policy(),
        }],
        eslint_contracts: vec![G3TsAstroMdxEslintPluginContractInput {
            app_root_rel_path: ".".to_owned(),
            config: eslint_config(),
        }],
        missing_component_map_sources: Vec::new(),
        eslint_directives: Vec::new(),
    }
}

fn package() -> G3TsAstroPackageSurfaceState {
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            package_name: Some("landing".to_owned()),
            dependencies: Vec::new(),
            dev_dependencies: vec!["eslint-plugin-mdx".to_owned()],
            script_names: Vec::new(),
            script_bodies: Vec::new(),
            script_commands: Vec::new(),
            script_tool_invocations: Vec::new(),
            script_parse_blockers: Vec::new(),
        },
    }
}

fn astro_policy() -> G3TsAstroMdxPolicySurfaceState {
    G3TsAstroMdxPolicySurfaceState::Parsed {
        snapshot: G3TsAstroMdxPolicySnapshot {
            rel_path: "guardrail3-ts.toml".to_owned(),
            content_root: Some("src/content".to_owned()),
            mdx_component_maps: vec!["src/mdx-components".to_owned()],
        },
    }
}

fn eslint_config() -> G3TsAstroMdxEslintSurfaceState {
    G3TsAstroMdxEslintSurfaceState::Parsed {
        snapshot: G3TsAstroMdxEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            mdx_content_probe_present: true,
            mdx_content_plugins: vec!["mdx".to_owned()],
            mdx_content_plugin_package_names: BTreeMap::from([
                ("mdx".to_owned(), vec!["eslint-plugin-mdx".to_owned()]),
                (
                    "@eslint-community/eslint-comments".to_owned(),
                    vec!["@eslint-community/eslint-plugin-eslint-comments".to_owned()],
                ),
            ]),
            mdx_content_error_rules: vec![
                "mdx/remark".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
            ],
            mdx_content_warn_or_error_rules: vec![
                "mdx/remark".to_owned(),
                "astro-pipeline/mdx-component-imports-from-approved-map".to_owned(),
                "astro-pipeline/mdx-imports-only-approved-components".to_owned(),
                "astro-pipeline/no-raw-mdx-images".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            mdx_content_restricted_disable_patterns: vec![
                "mdx/remark".to_owned(),
                "astro-pipeline/mdx-component-imports-from-approved-map".to_owned(),
                "astro-pipeline/mdx-imports-only-approved-components".to_owned(),
                "astro-pipeline/no-raw-mdx-images".to_owned(),
            ],
            mdx_content_unused_disable_fail_closed: true,
            mdx_content_effective_mdx_component_map_rules: vec![
                "astro-pipeline/mdx-component-imports-from-approved-map".to_owned(),
            ],
            mdx_content_effective_named_component_import_rules: vec![
                "astro-pipeline/mdx-imports-only-approved-components".to_owned(),
            ],
            mdx_content_effective_no_raw_image_rules: vec![
                "astro-pipeline/no-raw-mdx-images".to_owned(),
            ],
            component_map_probe_present: true,
            component_map_plugin_package_names: BTreeMap::from([
                (
                    "astro-pipeline".to_owned(),
                    vec!["g3ts-eslint-plugin-astro-pipeline".to_owned()],
                ),
                (
                    "@eslint-community/eslint-comments".to_owned(),
                    vec!["@eslint-community/eslint-plugin-eslint-comments".to_owned()],
                ),
            ]),
            component_map_error_rules: vec![
                "astro-pipeline/mdx-component-map-no-raw-ui-exports".to_owned(),
                "astro-pipeline/mdx-component-wrapper-requires-zod-parse".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
            ],
            component_map_warn_or_error_rules: vec![
                "astro-pipeline/mdx-component-map-no-raw-ui-exports".to_owned(),
                "astro-pipeline/mdx-component-wrapper-requires-zod-parse".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            component_map_restricted_disable_patterns: vec![
                "astro-pipeline/mdx-component-map-no-raw-ui-exports".to_owned(),
                "astro-pipeline/mdx-component-wrapper-requires-zod-parse".to_owned(),
            ],
            component_map_unused_disable_fail_closed: true,
            component_map_effective_no_raw_ui_export_rules: vec![
                "astro-pipeline/mdx-component-map-no-raw-ui-exports".to_owned(),
            ],
            component_map_effective_wrapper_zod_parse_rules: vec![
                "astro-pipeline/mdx-component-wrapper-requires-zod-parse".to_owned(),
            ],
            component_map_probe_ignored: false,
            mdx_content_probe_ignored: false,
        },
    }
}
