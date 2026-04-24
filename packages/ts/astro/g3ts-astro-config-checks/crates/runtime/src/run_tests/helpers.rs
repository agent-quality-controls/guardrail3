use g3ts_astro_types::{
    G3TsAstroConfigChecksInput, G3TsAstroContentMode, G3TsAstroEslintPluginContractInput,
    G3TsAstroEslintSurfaceSnapshot, G3TsAstroEslintSurfaceState, G3TsAstroIntegrationContractInput,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};

pub(super) fn golden() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_check() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(false, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn fake_astro_check_text_only() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package_with_script("echo astro check && eslint .", true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn astro_check_wrapper_forms() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package_with_script(
                "npm exec -- astro check && npx --yes astro check",
                true,
                true,
                true,
                false,
            ),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_required_packages() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, false, false, false, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_plugin_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(false, false, true))],
    }
}

pub(super) fn missing_pipeline_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, false, true))],
    }
}

pub(super) fn missing_pipeline_rule_enforcement() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, false))],
    }
}

pub(super) fn missing_pipeline_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            parsed_eslint_with_pipeline_contract(
                true,
                PipelineLaneContract {
                    astro: PipelineLaneState::rules_without_scope_options(),
                    ts: PipelineLaneState::rules_without_scope_options(),
                    tsx: PipelineLaneState::rules_without_scope_options(),
                },
            ),
        )],
    }
}

pub(super) fn endpoint_only_pipeline_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            parsed_eslint_with_pipeline_contract(
                true,
                PipelineLaneContract {
                    astro: PipelineLaneState::rules_with_endpoint_scope(),
                    ts: PipelineLaneState::rules_with_endpoint_scope(),
                    tsx: PipelineLaneState::rules_with_endpoint_scope(),
                },
            ),
        )],
    }
}

pub(super) fn missing_content_data_module_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            parsed_eslint_with_pipeline_contract(
                true,
                PipelineLaneContract {
                    astro: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
                    ts: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
                    tsx: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
                },
            ),
        )],
    }
}

pub(super) fn missing_content_source_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            parsed_eslint_with_pipeline_contract(
                true,
                PipelineLaneContract {
                    astro: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
                    ts: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
                    tsx: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
                },
            ),
        )],
    }
}

pub(super) fn route_only_pipeline_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, false),
        )],
        eslint_contracts: vec![eslint_contract(
            true,
            parsed_eslint_with_pipeline_contract(
                true,
                PipelineLaneContract {
                    astro: PipelineLaneState::rules_with_scope(),
                    ts: PipelineLaneState::disabled(),
                    tsx: PipelineLaneState::disabled(),
                },
            ),
        )],
    }
}

pub(super) fn optional_contracts_not_required() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            false,
            parsed_package(true, true, false, true, false),
        )],
        eslint_contracts: vec![eslint_contract(false, parsed_eslint(true, false, false))],
    }
}

pub(super) fn velite_package_present() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            parsed_package(true, true, true, true, true),
        )],
        eslint_contracts: vec![eslint_contract(true, parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_package_eslint_and_astro_config_surfaces() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(
            true,
            G3TsAstroPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
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
    requires_source_pipeline_linting: bool,
    package: G3TsAstroPackageSurfaceState,
) -> G3TsAstroIntegrationContractInput {
    G3TsAstroIntegrationContractInput {
        app_root_rel_path: ".".to_owned(),
        content_mode: G3TsAstroContentMode::BuildCollections,
        package,
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
    has_pipeline_plugin: bool,
    has_velite_package: bool,
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
        has_pipeline_plugin,
        has_velite_package,
    )
}

fn parsed_package_with_script(
    script_body: &str,
    has_astro_package: bool,
    has_astro_plugin: bool,
    has_pipeline_plugin: bool,
    has_velite_package: bool,
) -> G3TsAstroPackageSurfaceState {
    let mut dev_dependencies = Vec::new();
    if has_astro_package {
        dev_dependencies.push("astro".to_owned());
    }
    if has_astro_plugin {
        dev_dependencies.push("eslint-plugin-astro".to_owned());
    }
    if has_pipeline_plugin {
        dev_dependencies.push("eslint-plugin-astro-pipeline".to_owned());
    }
    if has_velite_package {
        dev_dependencies.push("velite".to_owned());
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
    let lane_state = if !has_pipeline_plugin {
        PipelineLaneState::disabled()
    } else if has_required_pipeline_rules {
        PipelineLaneState::rules_with_scope()
    } else {
        PipelineLaneState::plugin_only()
    };

    parsed_eslint_with_pipeline_contract(
        has_astro_plugin,
        PipelineLaneContract {
            astro: lane_state,
            ts: lane_state,
            tsx: lane_state,
        },
    )
}

fn parsed_eslint_with_pipeline_contract(
    has_astro_plugin: bool,
    pipeline_contract: PipelineLaneContract,
) -> G3TsAstroEslintSurfaceState {
    let mut astro_source_plugins = Vec::new();
    let mut ts_source_plugins = Vec::new();
    let mut tsx_source_plugins = Vec::new();
    if has_astro_plugin {
        astro_source_plugins.push("astro".to_owned());
        ts_source_plugins.push("astro".to_owned());
        tsx_source_plugins.push("astro".to_owned());
    }
    if pipeline_contract.astro.has_plugin {
        astro_source_plugins.push("astro-pipeline".to_owned());
    }
    if pipeline_contract.ts.has_plugin {
        ts_source_plugins.push("astro-pipeline".to_owned());
    }
    if pipeline_contract.tsx.has_plugin {
        tsx_source_plugins.push("astro-pipeline".to_owned());
    }
    let astro_source_error_rules = pipeline_contract.astro.error_rules();
    let ts_source_error_rules = pipeline_contract.ts.error_rules();
    let tsx_source_error_rules = pipeline_contract.tsx.error_rules();

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
            astro_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .astro
                .effective_route_scoped_rules(),
            ts_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .ts
                .effective_route_scoped_rules(),
            tsx_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .tsx
                .effective_route_scoped_rules(),
            astro_source_effective_content_data_pipeline_rules: pipeline_contract
                .astro
                .effective_content_data_rules(),
            ts_source_effective_content_data_pipeline_rules: pipeline_contract
                .ts
                .effective_content_data_rules(),
            tsx_source_effective_content_data_pipeline_rules: pipeline_contract
                .tsx
                .effective_content_data_rules(),
            astro_source_effective_content_source_pipeline_rules: pipeline_contract
                .astro
                .effective_content_source_rules(),
            ts_source_effective_content_source_pipeline_rules: pipeline_contract
                .ts
                .effective_content_source_rules(),
            tsx_source_effective_content_source_pipeline_rules: pipeline_contract
                .tsx
                .effective_content_source_rules(),
        },
    }
}

#[derive(Clone, Copy)]
struct PipelineLaneContract {
    astro: PipelineLaneState,
    ts: PipelineLaneState,
    tsx: PipelineLaneState,
}

#[derive(Clone, Copy)]
struct PipelineLaneState {
    has_plugin: bool,
    has_required_rules: bool,
    scope_kind: ScopeKind,
    has_content_data_scope: bool,
    has_content_source_scope: bool,
}

impl PipelineLaneState {
    const fn disabled() -> Self {
        Self {
            has_plugin: false,
            has_required_rules: false,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
        }
    }

    const fn plugin_only() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: false,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
        }
    }

    const fn rules_without_scope_options() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
        }
    }

    const fn rules_with_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: true,
            has_content_source_scope: true,
        }
    }

    const fn rules_with_endpoint_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Endpoint,
            has_content_data_scope: true,
            has_content_source_scope: true,
        }
    }

    const fn rules_with_scope_but_without_content_data_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: false,
            has_content_source_scope: true,
        }
    }

    const fn rules_with_scope_but_without_content_source_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: true,
            has_content_source_scope: false,
        }
    }

    fn error_rules(self) -> Vec<String> {
        if !self.has_required_rules {
            return Vec::new();
        }

        vec![
            "astro-pipeline/no-authored-content-fs-read".to_owned(),
            "astro-pipeline/no-authored-content-glob".to_owned(),
            "astro-pipeline/no-authored-content-imports".to_owned(),
            "astro-pipeline/no-content-data-modules-in-routes".to_owned(),
            "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
            "astro-pipeline/no-runtime-mdx-eval".to_owned(),
            "astro-pipeline/no-side-loader-imports".to_owned(),
            "astro-pipeline/no-velite-imports".to_owned(),
        ]
    }

    fn effective_route_scoped_rules(self) -> Vec<String> {
        if !self.has_required_rules || matches!(self.scope_kind, ScopeKind::None) {
            return Vec::new();
        }

        vec![
            "astro-pipeline/no-authored-content-fs-read".to_owned(),
            "astro-pipeline/no-authored-content-glob".to_owned(),
            "astro-pipeline/no-authored-content-imports".to_owned(),
            "astro-pipeline/no-content-data-modules-in-routes".to_owned(),
            "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
            "astro-pipeline/no-side-loader-imports".to_owned(),
            "astro-pipeline/no-velite-imports".to_owned(),
        ]
    }

    fn effective_content_data_rules(self) -> Vec<String> {
        if !self.has_required_rules || !self.has_content_data_scope {
            return Vec::new();
        }

        vec!["astro-pipeline/no-content-data-modules-in-routes".to_owned()]
    }

    fn effective_content_source_rules(self) -> Vec<String> {
        if !self.has_required_rules || !self.has_content_source_scope {
            return Vec::new();
        }

        vec![
            "astro-pipeline/no-authored-content-fs-read".to_owned(),
            "astro-pipeline/no-authored-content-glob".to_owned(),
            "astro-pipeline/no-authored-content-imports".to_owned(),
        ]
    }
}

#[derive(Clone, Copy)]
enum ScopeKind {
    None,
    Route,
    Endpoint,
}
