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
                "syncpack".to_owned(),
                "@astrojs/react".to_owned(),
                "@astrojs/mdx".to_owned(),
                "@astrojs/sitemap".to_owned(),
                "g3ts-astro-sitemap-auditor".to_owned(),
                "astro-robots".to_owned(),
                "g3ts-astro-robots-auditor".to_owned(),
                "@nuasite/checks".to_owned(),
                "g3ts-astro-llms-generator".to_owned(),
                "g3ts-astro-llms-auditor".to_owned(),
                "@eslint-community/eslint-plugin-eslint-comments".to_owned(),
            ],
            optional_dependencies: Vec::new(),
            peer_dependencies: Vec::new(),
            script_names: vec![
                "build".to_owned(),
                "check".to_owned(),
                "lint".to_owned(),
                "lint:packages".to_owned(),
                "validate".to_owned(),
            ],
            script_bodies: vec![
                ("build".to_owned(), "astro build".to_owned()),
                ("check".to_owned(), "astro check".to_owned()),
                ("lint".to_owned(), "eslint --max-warnings 0 .".to_owned()),
                ("lint:packages".to_owned(), "syncpack lint".to_owned()),
                (
                    "validate".to_owned(),
                    "pnpm run lint && pnpm run lint:packages && pnpm run check && pnpm run build"
                        .to_owned(),
                ),
            ],
            script_commands: vec![
                G3TsAstroPackageScriptCommand {
                    script_name: "build".to_owned(),
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "check".to_owned(),
                    invocation: "astro check".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["check".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "lint".to_owned(),
                    invocation: "eslint --max-warnings 0 .".to_owned(),
                    executable: "eslint".to_owned(),
                    args: vec!["--max-warnings".to_owned(), "0".to_owned(), ".".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "lint:packages".to_owned(),
                    invocation: "syncpack lint".to_owned(),
                    executable: "syncpack".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "pnpm run lint".to_owned(),
                    executable: "pnpm".to_owned(),
                    args: vec!["run".to_owned(), "lint".to_owned()],
                    preceded_by: None,
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "pnpm run lint:packages".to_owned(),
                    executable: "pnpm".to_owned(),
                    args: vec!["run".to_owned(), "lint:packages".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "pnpm run check".to_owned(),
                    executable: "pnpm".to_owned(),
                    args: vec!["run".to_owned(), "check".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptCommand {
                    script_name: "validate".to_owned(),
                    invocation: "pnpm run build".to_owned(),
                    executable: "pnpm".to_owned(),
                    args: vec!["run".to_owned(), "build".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
            ],
            script_tool_invocations: vec![
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "build".to_owned(),
                    command_index: 0,
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "check".to_owned(),
                    command_index: 0,
                    invocation: "astro check".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["check".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "lint".to_owned(),
                    command_index: 0,
                    invocation: "eslint --max-warnings 0 .".to_owned(),
                    executable: "eslint".to_owned(),
                    args: vec!["--max-warnings".to_owned(), "0".to_owned(), ".".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "lint:packages".to_owned(),
                    command_index: 0,
                    invocation: "syncpack lint".to_owned(),
                    executable: "syncpack".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 0,
                    invocation: "pnpm run lint".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 1,
                    invocation: "pnpm run lint:packages".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["lint:packages".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 2,
                    invocation: "pnpm run check".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["check".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 3,
                    invocation: "pnpm run build".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: None,
                },
            ],
            script_all_tool_invocations: vec![
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "build".to_owned(),
                    command_index: 0,
                    invocation: "astro build".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "check".to_owned(),
                    command_index: 0,
                    invocation: "astro check".to_owned(),
                    executable: "astro".to_owned(),
                    args: vec!["check".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "lint".to_owned(),
                    command_index: 0,
                    invocation: "eslint --max-warnings 0 .".to_owned(),
                    executable: "eslint".to_owned(),
                    args: vec!["--max-warnings".to_owned(), "0".to_owned(), ".".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "lint:packages".to_owned(),
                    command_index: 0,
                    invocation: "syncpack lint".to_owned(),
                    executable: "syncpack".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: None,
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 0,
                    invocation: "pnpm run lint".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 1,
                    invocation: "pnpm run lint:packages".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["lint:packages".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 2,
                    invocation: "pnpm run check".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["check".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                },
                G3TsAstroPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 3,
                    invocation: "pnpm run build".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["build".to_owned()],
                    preceded_by: Some(
                        g3ts_astro_setup_types::G3TsAstroPackageScriptCommandSeparator::And,
                    ),
                    followed_by: None,
                },
            ],
            script_parse_blockers: Vec::new(),
        },
    }
}

pub(super) fn parsed_package_mut(
    input: &mut G3TsAstroSetupConfigChecksInput,
) -> &mut G3TsAstroPackageSurfaceSnapshot {
    let G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        panic!("golden package should be parsed");
    };

    snapshot
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
                integration(
                    "g3ts-astro-sitemap-auditor",
                    Some(G3TsAstroStaticValue::Object(vec![])),
                ),
                integration("astro-robots", None),
                integration(
                    "g3ts-astro-robots-auditor",
                    Some(G3TsAstroStaticValue::Object(vec![])),
                ),
                integration(
                    "@nuasite/checks",
                    Some(G3TsAstroStaticValue::Object(vec![])),
                ),
                integration(
                    "g3ts-astro-llms-generator",
                    Some(G3TsAstroStaticValue::Object(vec![])),
                ),
                integration(
                    "g3ts-astro-llms-auditor",
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
            astro_source_plugins: vec![
                "astro".to_owned(),
                "@eslint-community/eslint-comments".to_owned(),
            ],
            ts_source_plugins: vec!["@eslint-community/eslint-comments".to_owned()],
            tsx_source_plugins: vec!["@eslint-community/eslint-comments".to_owned()],
            astro_source_plugin_meta_names: BTreeMap::new(),
            ts_source_plugin_meta_names: BTreeMap::new(),
            tsx_source_plugin_meta_names: BTreeMap::new(),
            astro_source_plugin_package_names: BTreeMap::from([
                ("astro".to_owned(), vec!["eslint-plugin-astro".to_owned()]),
                (
                    "@eslint-community/eslint-comments".to_owned(),
                    vec!["@eslint-community/eslint-plugin-eslint-comments".to_owned()],
                ),
            ]),
            ts_source_plugin_package_names: BTreeMap::from([(
                "@eslint-community/eslint-comments".to_owned(),
                vec!["@eslint-community/eslint-plugin-eslint-comments".to_owned()],
            )]),
            tsx_source_plugin_package_names: BTreeMap::from([(
                "@eslint-community/eslint-comments".to_owned(),
                vec!["@eslint-community/eslint-plugin-eslint-comments".to_owned()],
            )]),
            astro_source_error_rules: vec![
                "astro/valid-compile".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
            ],
            ts_source_error_rules: vec![
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
            ],
            tsx_source_error_rules: vec![
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
            ],
            astro_source_warn_or_error_rules: vec![
                "astro/valid-compile".to_owned(),
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            ts_source_warn_or_error_rules: vec![
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            tsx_source_warn_or_error_rules: vec![
                "@eslint-community/eslint-comments/require-description".to_owned(),
                "@eslint-community/eslint-comments/no-unused-disable".to_owned(),
                "@eslint-community/eslint-comments/no-restricted-disable".to_owned(),
            ],
            astro_source_restricted_disable_patterns: vec![
                "astro/valid-compile".to_owned(),
                "@eslint-community/eslint-comments/*".to_owned(),
            ],
            ts_source_restricted_disable_patterns: vec![
                "@eslint-community/eslint-comments/*".to_owned(),
            ],
            tsx_source_restricted_disable_patterns: vec![
                "@eslint-community/eslint-comments/*".to_owned(),
            ],
            astro_source_unused_disable_fail_closed: true,
            ts_source_unused_disable_fail_closed: true,
            tsx_source_unused_disable_fail_closed: true,
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
