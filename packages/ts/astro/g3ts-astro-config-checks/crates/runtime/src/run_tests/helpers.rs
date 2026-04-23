use g3ts_astro_types::{
    G3TsAstroConfigChecksInput, G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroContentMode, G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroIntegrationContractInput, G3TsAstroOutputMode,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};

pub(super) fn golden() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(true, true, true, true, true),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_check() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(false, true, true, true, true),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn fake_astro_check_text_only() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package_with_script(
                "echo astro check && eslint .",
                true,
                true,
                true,
                true,
            ),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn astro_check_wrapper_forms() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package_with_script(
                "npm exec -- astro check && npx --yes astro check",
                true,
                true,
                true,
                true,
            ),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_required_packages() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(true, false, false, false, false),
            parsed_astro_config(false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_plugin_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(true, true, true, true, true),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(false, false, true))],
    }
}

pub(super) fn missing_pipeline_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(true, true, true, true, true),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, false, true))],
    }
}

pub(super) fn missing_pipeline_rule_enforcement() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            parsed_package(true, true, true, true, true),
            parsed_astro_config(true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, false))],
    }
}

pub(super) fn optional_contracts_not_required() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            false,
            false,
            parsed_package(true, true, false, false, true),
            parsed_astro_config(false),
        )],
        eslint_contracts: vec![eslint_contract(false, parsed_eslint(true, false, false))],
    }
}

pub(super) fn missing_package_eslint_and_astro_config_surfaces() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            true,
            G3TsAstroPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
            },
            G3TsAstroConfigSurfaceState::Missing {
                rel_path: "astro.config.*".to_owned(),
            },
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            G3TsAstroEslintSurfaceState::Missing {
                rel_path: "eslint.config.*".to_owned(),
            },
        )],
    }
}

fn integration_contract(
    requires_render_validator: bool,
    requires_source_pipeline_linting: bool,
    package: G3TsAstroPackageSurfaceState,
    astro_config: G3TsAstroConfigSurfaceState,
) -> G3TsAstroIntegrationContractInput {
    G3TsAstroIntegrationContractInput {
        app_root_rel_path: ".".to_owned(),
        content_mode: G3TsAstroContentMode::BuildCollections,
        package,
        astro_config,
        requires_render_validator,
        requires_source_pipeline_linting,
    }
}

fn eslint_contract(
    requires_source_pipeline_linting: bool,
    config: G3TsAstroEslintSurfaceState,
) -> G3TsAstroEslintPluginContractInput {
    G3TsAstroEslintPluginContractInput {
        app_root_rel_path: ".".to_owned(),
        config,
        requires_source_pipeline_linting,
    }
}

fn parsed_package(
    has_astro_check: bool,
    has_astro_package: bool,
    has_astro_plugin: bool,
    has_render_validator: bool,
    has_pipeline_plugin: bool,
) -> G3TsAstroPackageSurfaceState {
    let script_body = if has_astro_check {
        "astro check && eslint ."
    } else {
        "eslint ."
    };

    parsed_package_with_script(
        script_body,
        has_astro_package,
        has_astro_plugin,
        has_render_validator,
        has_pipeline_plugin,
    )
}

fn parsed_package_with_script(
    script_body: &str,
    has_astro_package: bool,
    has_astro_plugin: bool,
    has_render_validator: bool,
    has_pipeline_plugin: bool,
) -> G3TsAstroPackageSurfaceState {
    let mut dev_dependencies = Vec::new();
    if has_astro_package {
        dev_dependencies.push("astro".to_owned());
    }
    if has_astro_plugin {
        dev_dependencies.push("eslint-plugin-astro".to_owned());
    }
    if has_render_validator {
        dev_dependencies.push("@nuasite/checks".to_owned());
    }
    if has_pipeline_plugin {
        dev_dependencies.push("eslint-plugin-astro-pipeline".to_owned());
    }

    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            dependencies: Vec::new(),
            dev_dependencies,
            script_names: vec!["check".to_owned()],
            script_bodies: vec![("check".to_owned(), script_body.to_owned())],
        },
    }
}

fn parsed_eslint(
    has_astro_plugin: bool,
    has_pipeline_plugin: bool,
    has_required_pipeline_rules: bool,
) -> G3TsAstroEslintSurfaceState {
    let mut astro_source_plugins = Vec::new();
    let mut ts_source_plugins = Vec::new();
    let mut tsx_source_plugins = Vec::new();
    let mut astro_source_error_rules = Vec::new();
    let mut ts_source_error_rules = Vec::new();
    let mut tsx_source_error_rules = Vec::new();
    if has_astro_plugin {
        astro_source_plugins.push("astro".to_owned());
        ts_source_plugins.push("astro".to_owned());
        tsx_source_plugins.push("astro".to_owned());
    }
    if has_pipeline_plugin {
        astro_source_plugins.push("astro-pipeline".to_owned());
        ts_source_plugins.push("astro-pipeline".to_owned());
        tsx_source_plugins.push("astro-pipeline".to_owned());
    }
    if has_required_pipeline_rules {
        astro_source_error_rules = vec![
            "astro-pipeline/no-authored-content-fs-read".to_owned(),
            "astro-pipeline/no-authored-content-glob".to_owned(),
            "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
            "astro-pipeline/no-runtime-mdx-eval".to_owned(),
            "astro-pipeline/no-side-loader-imports".to_owned(),
        ];
        ts_source_error_rules = astro_source_error_rules.clone();
        tsx_source_error_rules = astro_source_error_rules.clone();
    }

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            astro_source_probe_present: true,
            ts_source_probe_present: true,
            tsx_source_probe_present: true,
            astro_source_plugins,
            ts_source_plugins,
            tsx_source_plugins,
            astro_source_error_rules,
            ts_source_error_rules,
            tsx_source_error_rules,
        },
    }
}

fn parsed_astro_config(has_render_validator_integration: bool) -> G3TsAstroConfigSurfaceState {
    let mut integration_modules = Vec::new();
    if has_render_validator_integration {
        integration_modules.push("@nuasite/checks".to_owned());
    }

    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: "astro.config.mjs".to_owned(),
            output_mode: Some(G3TsAstroOutputMode::Server),
            adapter_module: Some("@astrojs/node".to_owned()),
            integration_modules,
        },
    }
}
