use g3ts_astro_setup_types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroIntegrationSnapshot, G3TsAstroOutputMode, G3TsAstroPackageScriptCommand,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroSetupConfigChecksInput,
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupEslintSurfaceSnapshot,
    G3TsAstroSetupEslintSurfaceState, G3TsAstroSetupIntegrationContractInput, G3TsAstroStaticValue,
    G3TsAstroSyncpackConfigSnapshot, G3TsAstroSyncpackConfigState, G3TsAstroSyncpackRequiredPin,
};
use std::collections::BTreeMap;

pub(super) fn golden() -> G3TsAstroSetupConfigChecksInput {
    G3TsAstroSetupConfigChecksInput {
        integration_contracts: vec![G3TsAstroSetupIntegrationContractInput {
            app_root_rel_path: ".".to_owned(),
            package: package(),
            syncpack_config: syncpack_config(),
            astro_config: astro_config(),
            required_syncpack_pins: required_syncpack_pins(),
            forbidden_syncpack_deps: vec!["next".to_owned(), "velite".to_owned()],
        }],
        eslint_contracts: vec![G3TsAstroSetupEslintPluginContractInput {
            app_root_rel_path: ".".to_owned(),
            config: eslint_config(),
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
                "astro".to_owned(),
                "@astrojs/check".to_owned(),
                "eslint-plugin-astro".to_owned(),
                "@astrojs/react".to_owned(),
                "@astrojs/mdx".to_owned(),
                "@astrojs/sitemap".to_owned(),
                "astro-robots".to_owned(),
                "@nuasite/checks".to_owned(),
            ],
            script_names: vec!["check".to_owned()],
            script_bodies: vec![("check".to_owned(), "astro check".to_owned())],
            script_commands: vec![G3TsAstroPackageScriptCommand {
                script_name: "check".to_owned(),
                invocation: "astro check".to_owned(),
                executable: "astro".to_owned(),
                args: vec!["check".to_owned()],
                preceded_by: None,
            }],
            script_tool_invocations: vec![G3TsAstroPackageScriptToolInvocation {
                script_name: "check".to_owned(),
                command_index: 0,
                invocation: "astro check".to_owned(),
                executable: "astro".to_owned(),
                args: vec!["check".to_owned()],
                preceded_by: None,
                followed_by: None,
            }],
            script_parse_blockers: Vec::new(),
        },
    }
}

fn syncpack_config() -> G3TsAstroSyncpackConfigState {
    G3TsAstroSyncpackConfigState::Parsed {
        snapshot: G3TsAstroSyncpackConfigSnapshot {
            rel_path: ".syncpackrc".to_owned(),
            source_covers_package_manifest: true,
            missing_required_stack_pins: Vec::new(),
            missing_forbidden_bans: Vec::new(),
        },
    }
}

fn astro_config() -> G3TsAstroConfigSurfaceState {
    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: "astro.config.mjs".to_owned(),
            site: Some("https://example.com".to_owned()),
            output: Some(G3TsAstroOutputMode::Static),
            integrations: vec![
                integration("@astrojs/react", None),
                integration("@astrojs/mdx", None),
                integration("@astrojs/sitemap", None),
                integration("astro-robots", None),
                integration(
                    "@nuasite/checks",
                    Some(G3TsAstroStaticValue::Object(vec![])),
                ),
            ],
            adapter: None,
        },
    }
}

fn eslint_config() -> G3TsAstroSetupEslintSurfaceState {
    G3TsAstroSetupEslintSurfaceState::Parsed {
        snapshot: G3TsAstroSetupEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            astro_source_probe_present: true,
            ts_source_probe_present: true,
            tsx_source_probe_present: true,
            astro_source_plugins: vec!["astro".to_owned()],
            ts_source_plugins: Vec::new(),
            tsx_source_plugins: Vec::new(),
            astro_source_plugin_meta_names: BTreeMap::new(),
            ts_source_plugin_meta_names: BTreeMap::new(),
            tsx_source_plugin_meta_names: BTreeMap::new(),
            astro_source_plugin_package_names: BTreeMap::from([(
                "astro".to_owned(),
                vec!["eslint-plugin-astro".to_owned()],
            )]),
            ts_source_plugin_package_names: BTreeMap::new(),
            tsx_source_plugin_package_names: BTreeMap::new(),
            astro_source_error_rules: vec!["astro/valid-compile".to_owned()],
            ts_source_error_rules: Vec::new(),
            tsx_source_error_rules: Vec::new(),
            astro_source_probe_ignored: false,
            ts_source_probe_ignored: false,
            tsx_source_probe_ignored: false,
        },
    }
}

fn required_syncpack_pins() -> Vec<G3TsAstroSyncpackRequiredPin> {
    vec![G3TsAstroSyncpackRequiredPin {
        dependency: "astro".to_owned(),
        version: "6.1.9".to_owned(),
    }]
}

fn integration(
    source_module: &str,
    first_arg: Option<G3TsAstroStaticValue>,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: Some(source_module.to_owned()),
        name: Some(source_module.to_owned()),
        imported_name: None,
        call: Some(G3TsAstroCallSnapshot { first_arg }),
    }
}
