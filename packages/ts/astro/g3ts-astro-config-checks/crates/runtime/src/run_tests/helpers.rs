use g3ts_astro_types::{
    G3TsAstroCallSnapshot, G3TsAstroConfigChecksInput, G3TsAstroConfigSurfaceSnapshot,
    G3TsAstroConfigSurfaceState, G3TsAstroContentMode, G3TsAstroEslintPluginContractInput,
    G3TsAstroEslintSurfaceSnapshot, G3TsAstroEslintSurfaceState, G3TsAstroIntegrationContractInput,
    G3TsAstroIntegrationSnapshot, G3TsAstroOutputMode, G3TsAstroPackageScriptCommand,
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptParseBlocker,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroPipelineRuleScopeSnapshot, G3TsAstroPolicySnapshot,
    G3TsAstroPolicySurfaceState, G3TsAstroStaticObjectProperty, G3TsAstroStaticValue,
    G3TsAstroSyncpackConfigSnapshot, G3TsAstroSyncpackConfigState, G3TsAstroSyncpackRequiredPin,
};
use package_script_command_parser::types::{
    PackageScriptCommand, PackageScriptCommandSeparator, PackageScriptParseFact,
    PackageScriptParseState, PackageScriptToolInvocation,
};
use std::collections::{BTreeMap, BTreeSet};

const FORBIDDEN_SYNCPACK_DEPS: [&str; 12] = [
    "next",
    "velite",
    "@astrojs/node",
    "eslint-plugin-astro-pipeline",
    "@codemint/astro-meta",
    "astro-seo",
    "astro-seo-meta",
    "astro-seo-schema",
    "contentlayer",
    "next-contentlayer",
    "@contentlayer/core",
    "@contentlayer/source-files",
];

#[derive(Clone)]
struct TestSyncpackVersionGroup {
    dependencies: Vec<String>,
    dependency_types: Vec<String>,
    packages: Vec<String>,
    specifier_types: Vec<String>,
    pin_version: Option<String>,
    is_banned: bool,
    is_ignored: bool,
}

pub(super) fn golden() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_check() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            false, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn fake_astro_check_text_only() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package_with_script(
            "echo astro check && syncpack lint && eslint .",
            true,
            true,
            true,
            false,
            true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn astro_check_wrapper_forms() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package_with_script(
            "npm exec -- astro check && npx --yes astro check && syncpack lint",
            true,
            true,
            true,
            false,
            true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_required_packages() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, false, false, false, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_astro_plugin_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(false, false, true))],
    }
}

pub(super) fn missing_pipeline_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, false, true))],
    }
}

pub(super) fn missing_pipeline_rule_enforcement() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, false))],
    }
}

pub(super) fn missing_pipeline_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_without_scope_options(),
                ts: PipelineLaneState::rules_without_scope_options(),
                tsx: PipelineLaneState::rules_without_scope_options(),
            },
        ))],
    }
}

pub(super) fn endpoint_only_pipeline_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_endpoint_scope(),
                ts: PipelineLaneState::rules_with_endpoint_scope(),
                tsx: PipelineLaneState::rules_with_endpoint_scope(),
            },
        ))],
    }
}

pub(super) fn endpoint_only_pipeline_scope_without_route_coverage() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_without_effective_route_coverage(),
                ts: PipelineLaneState::rules_without_effective_route_coverage(),
                tsx: PipelineLaneState::rules_without_effective_route_coverage(),
            },
        ))],
    }
}

pub(super) fn missing_content_data_module_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
                ts: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
                tsx: PipelineLaneState::rules_with_scope_but_without_content_data_scope(),
            },
        ))],
    }
}

pub(super) fn missing_content_source_scope_options() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
                ts: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
                tsx: PipelineLaneState::rules_with_scope_but_without_content_source_scope(),
            },
        ))],
    }
}

pub(super) fn missing_inline_public_content_rule() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope_but_without_inline_public_content_policy(
                ),
                ts: PipelineLaneState::rules_with_scope_but_without_inline_public_content_policy(),
                tsx: PipelineLaneState::rules_with_scope_but_without_inline_public_content_policy(),
            },
        ))],
    }
}

pub(super) fn route_only_pipeline_wiring() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope(),
                ts: PipelineLaneState::disabled(),
                tsx: PipelineLaneState::disabled(),
            },
        ))],
    }
}

pub(super) fn tsx_lane_missing_pipeline_effectiveness() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope(),
                ts: PipelineLaneState::rules_with_scope(),
                tsx: PipelineLaneState::rules_without_effective_route_coverage(),
            },
        ))],
    }
}

pub(super) fn astro_lane_missing_pipeline_effectiveness() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_without_effective_route_coverage(),
                ts: PipelineLaneState::rules_with_scope(),
                tsx: PipelineLaneState::rules_with_scope(),
            },
        ))],
    }
}

pub(super) fn ts_lane_missing_pipeline_effectiveness() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, false, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint_with_pipeline_contract(
            true,
            PipelineLaneContract {
                astro: PipelineLaneState::rules_with_scope(),
                ts: PipelineLaneState::rules_without_effective_route_coverage(),
                tsx: PipelineLaneState::rules_with_scope(),
            },
        ))],
    }
}

pub(super) fn missing_syncpack_config() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            G3TsAstroSyncpackConfigState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn unreadable_syncpack_config() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            G3TsAstroSyncpackConfigState::Unreadable {
                rel_path: ".syncpackrc".to_owned(),
                reason: "permission denied".to_owned(),
            },
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn malformed_syncpack_config() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            G3TsAstroSyncpackConfigState::ParseError {
                rel_path: ".syncpackrc".to_owned(),
                reason: "Syncpack config field `versionGroups` must be an array".to_owned(),
            },
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_source_excludes_package() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_source_and_groups(
                vec!["unrelated/package.json".to_owned()],
                required_syncpack_version_groups(),
            ),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn root_syncpack_package_source_does_not_cover_nested_app() -> G3TsAstroConfigChecksInput
{
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_for_app_with_syncpack(
            "apps/landing",
            nested_parsed_package(),
            syncpack_config_for_package_at_with_source_and_groups(
                ".syncpackrc",
                "apps/landing/package.json",
                Some("landing"),
                vec!["package.json".to_owned()],
                required_syncpack_version_groups(),
            ),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn root_syncpack_exact_source_covers_nested_app() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_for_app_with_syncpack(
            "apps/landing",
            nested_parsed_package(),
            syncpack_config_for_package_at_with_source_and_groups(
                ".syncpackrc",
                "apps/landing/package.json",
                Some("landing"),
                vec!["apps/landing/package.json".to_owned()],
                required_syncpack_version_groups(),
            ),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn local_syncpack_package_source_covers_nested_app() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_for_app_with_syncpack(
            "apps/landing",
            nested_parsed_package(),
            syncpack_config_for_package_at_with_source_and_groups(
                "apps/landing/.syncpackrc",
                "apps/landing/package.json",
                Some("landing"),
                vec!["package.json".to_owned()],
                required_syncpack_version_groups(),
            ),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_missing_stack_pin() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    groups.retain(|group| {
        !group
            .dependencies
            .iter()
            .any(|dependency| dependency == "astro")
    });

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_wrong_stack_pin() -> G3TsAstroConfigChecksInput {
    syncpack_wrong_pin_for_dependency("astro", "6.1.8")
}

pub(super) fn syncpack_wrong_astro_pipeline_stack_pin() -> G3TsAstroConfigChecksInput {
    syncpack_wrong_pin_for_dependency("g3ts-eslint-plugin-astro-pipeline", "0.1.4")
}

fn syncpack_wrong_pin_for_dependency(
    dependency_name: &str,
    wrong_version: &str,
) -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    for group in &mut groups {
        if group
            .dependencies
            .iter()
            .any(|dependency| dependency == dependency_name)
        {
            group.pin_version = Some(wrong_version.to_owned());
        }
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_shadowed_stack_pin() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    groups.insert(
        0,
        TestSyncpackVersionGroup {
            dependencies: vec!["astro".to_owned()],
            dependency_types: vec!["prod".to_owned(), "dev".to_owned()],
            packages: Vec::new(),
            specifier_types: Vec::new(),
            pin_version: Some("6.1.8".to_owned()),
            is_banned: false,
            is_ignored: false,
        },
    );

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_scoped_away_stack_pin() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    for group in &mut groups {
        if group
            .dependencies
            .iter()
            .any(|dependency| dependency == "astro")
        {
            group.packages = vec!["other-package".to_owned()];
        }
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_specifier_scoped_stack_pin() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    for group in &mut groups {
        if group
            .dependencies
            .iter()
            .any(|dependency| dependency == "astro")
        {
            group.specifier_types = vec!["!exact".to_owned()];
        }
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_catch_all_forbidden_ban() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups()
        .into_iter()
        .filter(|group| !group.is_banned)
        .collect::<Vec<_>>();
    groups.push(TestSyncpackVersionGroup {
        dependencies: vec!["**".to_owned()],
        dependency_types: vec!["**".to_owned()],
        packages: Vec::new(),
        specifier_types: Vec::new(),
        pin_version: None,
        is_banned: true,
        is_ignored: false,
    });

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_shadowed_forbidden_ban() -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    let next_ban_index = groups
        .iter()
        .position(|group| {
            group.is_banned
                && group
                    .dependencies
                    .iter()
                    .any(|dependency| dependency == "next")
        })
        .expect("next ban should exist");
    let canonical_next_ban = groups.remove(next_ban_index);
    groups.insert(
        next_ban_index,
        TestSyncpackVersionGroup {
            dependencies: vec!["next".to_owned()],
            dependency_types: vec![
                "prod".to_owned(),
                "dev".to_owned(),
                "optional".to_owned(),
                "peer".to_owned(),
            ],
            packages: Vec::new(),
            specifier_types: Vec::new(),
            pin_version: None,
            is_banned: false,
            is_ignored: false,
        },
    );
    groups.push(canonical_next_ban);

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_scoped_away_forbidden_ban() -> G3TsAstroConfigChecksInput {
    syncpack_mutated_next_ban(|group| {
        group.packages = vec!["other-package".to_owned()];
    })
}

pub(super) fn syncpack_specifier_scoped_forbidden_ban() -> G3TsAstroConfigChecksInput {
    syncpack_mutated_next_ban(|group| {
        group.specifier_types = vec!["!exact".to_owned()];
    })
}

pub(super) fn syncpack_wrong_forbidden_ban_dependency_types() -> G3TsAstroConfigChecksInput {
    syncpack_mutated_next_ban(|group| {
        group.dependency_types = vec!["prod".to_owned(), "dev".to_owned()];
    })
}

pub(super) fn syncpack_ignored_forbidden_ban() -> G3TsAstroConfigChecksInput {
    syncpack_mutated_next_ban(|group| {
        group.is_ignored = true;
    })
}

pub(super) fn syncpack_pinned_forbidden_ban() -> G3TsAstroConfigChecksInput {
    syncpack_mutated_next_ban(|group| {
        group.pin_version = Some("0.0.0".to_owned());
    })
}

fn syncpack_mutated_next_ban(
    mut mutate: impl FnMut(&mut TestSyncpackVersionGroup),
) -> G3TsAstroConfigChecksInput {
    let mut groups = required_syncpack_version_groups();
    for group in &mut groups {
        if group.is_banned
            && group
                .dependencies
                .iter()
                .any(|dependency| dependency == "next")
        {
            mutate(group);
        }
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn velite_package_with_syncpack_ban() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract(parsed_package(
            true, true, true, true, true, true, true,
        ))],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn syncpack_missing_forbidden_ban() -> G3TsAstroConfigChecksInput {
    syncpack_missing_forbidden_ban_for("next")
}

pub(super) fn syncpack_missing_astro_seo_ban() -> G3TsAstroConfigChecksInput {
    syncpack_missing_forbidden_ban_for("astro-seo")
}

pub(super) fn syncpack_missing_contentlayer_ban() -> G3TsAstroConfigChecksInput {
    syncpack_missing_forbidden_ban_for("contentlayer")
}

pub(super) fn syncpack_missing_forbidden_ban_named(
    forbidden_dependency: &str,
) -> G3TsAstroConfigChecksInput {
    syncpack_missing_forbidden_ban_for(forbidden_dependency)
}

fn syncpack_missing_forbidden_ban_for(forbidden_dependency: &str) -> G3TsAstroConfigChecksInput {
    let groups = required_syncpack_version_groups()
        .into_iter()
        .map(|mut group| {
            if group.is_banned {
                group
                    .dependencies
                    .retain(|dependency| dependency != forbidden_dependency);
            }
            group
        })
        .collect();

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            parsed_package(true, true, true, true, false, true, true),
            syncpack_config_with_groups(groups),
        )],
        eslint_contracts: vec![eslint_contract(parsed_eslint(true, true, true))],
    }
}

pub(super) fn missing_package_eslint_and_astro_config_surfaces() -> G3TsAstroConfigChecksInput {
    G3TsAstroConfigChecksInput {
        integration_contracts: vec![integration_contract_with_syncpack(
            G3TsAstroPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
            },
            G3TsAstroSyncpackConfigState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        )],
        eslint_contracts: vec![eslint_contract(G3TsAstroEslintSurfaceState::Missing {
            rel_path: "eslint.config.*".to_owned(),
        })],
    }
}

fn integration_contract(
    package: G3TsAstroPackageSurfaceState,
) -> G3TsAstroIntegrationContractInput {
    integration_contract_with_syncpack(package, syncpack_config())
}

fn integration_contract_with_syncpack(
    package: G3TsAstroPackageSurfaceState,
    syncpack_config: G3TsAstroSyncpackConfigState,
) -> G3TsAstroIntegrationContractInput {
    integration_contract_for_app_with_syncpack(".", package, syncpack_config)
}

fn integration_contract_for_app_with_syncpack(
    app_root_rel_path: &str,
    package: G3TsAstroPackageSurfaceState,
    syncpack_config: G3TsAstroSyncpackConfigState,
) -> G3TsAstroIntegrationContractInput {
    G3TsAstroIntegrationContractInput {
        app_root_rel_path: app_root_rel_path.to_owned(),
        content_mode: G3TsAstroContentMode::BuildCollections,
        route_page_paths: vec!["src/pages/index.astro".to_owned()],
        endpoint_paths: vec!["src/pages/rss.ts".to_owned()],
        content_adapter_source_paths: vec!["src/lib/content/index.ts".to_owned()],
        package,
        syncpack_config,
        astro_policy: astro_policy(),
        astro_config: astro_config(),
        llms_txt_rel_path: Some("public/llms.txt".to_owned()),
        required_syncpack_pins: required_syncpack_pins(),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

fn astro_policy() -> G3TsAstroPolicySurfaceState {
    G3TsAstroPolicySurfaceState::Parsed {
        snapshot: G3TsAstroPolicySnapshot {
            rel_path: "guardrail3-ts.toml".to_owned(),
            profile: Some("strict-local-content".to_owned()),
            content_routes: vec!["src/pages/**/*.astro".to_owned()],
            non_content_routes: vec!["src/pages/404.astro".to_owned()],
            endpoints: vec!["src/pages/**/*.ts".to_owned()],
            content_root: Some("src/content".to_owned()),
            content_adapter: Some("src/lib/content".to_owned()),
            forbidden_state: vec![
                ".next/**".to_owned(),
                ".velite/**".to_owned(),
                ".contentlayer/**".to_owned(),
            ],
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
                integration("@astrojs/react", None, None),
                integration("@astrojs/mdx", None, None),
                integration("@astrojs/sitemap", None, None),
                integration("astro-robots", None, None),
                integration(
                    "@nuasite/checks",
                    None,
                    Some(G3TsAstroStaticValue::Object(vec![
                        property("mode", G3TsAstroStaticValue::String("full".to_owned())),
                        property("failOnError", G3TsAstroStaticValue::Bool(true)),
                        property("failOnWarning", G3TsAstroStaticValue::Bool(true)),
                        property("reportJson", G3TsAstroStaticValue::Bool(true)),
                        property("ai", G3TsAstroStaticValue::Bool(false)),
                        property(
                            "customChecks",
                            G3TsAstroStaticValue::Array(vec![
                                G3TsAstroStaticValue::ImportedIdentifier {
                                    local_name: "structuredDataPresentCheck".to_owned(),
                                    source_module: Some("g3ts-astro-nuasite-checks".to_owned()),
                                    imported_name: Some("structuredDataPresentCheck".to_owned()),
                                },
                            ]),
                        ),
                    ])),
                ),
            ],
            adapter: None,
        },
    }
}

fn integration(
    source_module: &str,
    imported_name: Option<&str>,
    first_arg: Option<G3TsAstroStaticValue>,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: Some(source_module.to_owned()),
        name: Some(source_module.to_owned()),
        imported_name: imported_name.map(str::to_owned),
        call: Some(G3TsAstroCallSnapshot { first_arg }),
    }
}

fn property(key: &str, value: G3TsAstroStaticValue) -> G3TsAstroStaticObjectProperty {
    G3TsAstroStaticObjectProperty {
        key: key.to_owned(),
        value,
    }
}

fn eslint_contract(config: G3TsAstroEslintSurfaceState) -> G3TsAstroEslintPluginContractInput {
    G3TsAstroEslintPluginContractInput {
        app_root_rel_path: ".".to_owned(),
        config,
    }
}

fn parsed_package(
    has_astro_check: bool,
    has_astro_package: bool,
    has_astro_plugin: bool,
    has_pipeline_plugin: bool,
    has_velite_package: bool,
    has_syncpack_package: bool,
    has_syncpack_script: bool,
) -> G3TsAstroPackageSurfaceState {
    let script_body = if has_astro_check {
        if has_syncpack_script {
            "astro check && syncpack lint && astro build && eslint ."
        } else {
            "astro check && astro build && eslint ."
        }
    } else {
        "eslint ."
    };

    parsed_package_with_script(
        script_body,
        has_astro_package,
        has_astro_plugin,
        has_pipeline_plugin,
        has_velite_package,
        has_syncpack_package,
    )
}

fn parsed_package_with_script(
    script_body: &str,
    has_astro_package: bool,
    has_astro_plugin: bool,
    has_pipeline_plugin: bool,
    has_velite_package: bool,
    has_syncpack_package: bool,
) -> G3TsAstroPackageSurfaceState {
    let mut dev_dependencies = Vec::new();
    if has_astro_package {
        dev_dependencies.push("astro".to_owned());
        dev_dependencies.extend(
            [
                "@astrojs/react",
                "@astrojs/mdx",
                "@astrojs/check",
                "@astrojs/sitemap",
                "astro-robots",
                "@nuasite/checks",
                "g3ts-astro-nuasite-checks",
                "schema-dts",
                "react",
                "react-dom",
                "@types/react",
                "@types/react-dom",
                "typescript",
                "eslint-plugin-i18next",
                "eslint-plugin-mdx",
            ]
            .into_iter()
            .map(str::to_owned),
        );
    }
    if has_astro_plugin {
        dev_dependencies.push("eslint-plugin-astro".to_owned());
    }
    if has_pipeline_plugin {
        dev_dependencies.push("g3ts-eslint-plugin-astro-pipeline".to_owned());
    }
    if has_velite_package {
        dev_dependencies.push("velite".to_owned());
    }
    if has_syncpack_package {
        dev_dependencies.push("syncpack".to_owned());
    }

    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            package_name: Some("landing".to_owned()),
            dependencies: Vec::new(),
            dev_dependencies,
            script_names: vec!["check".to_owned(), "build".to_owned()],
            script_bodies: vec![
                ("check".to_owned(), script_body.to_owned()),
                ("build".to_owned(), "astro build".to_owned()),
            ],
            script_commands: script_commands_for_test("check", script_body)
                .into_iter()
                .chain(script_commands_for_test("build", "astro build"))
                .collect(),
            script_tool_invocations: script_tool_invocations_for_test("check", script_body)
                .into_iter()
                .chain(script_tool_invocations_for_test("build", "astro build"))
                .collect(),
            script_parse_blockers: script_parse_blockers_for_test("check", script_body),
            safely_runs_astro_check: package_script_command_parser::has_safe_tool_invocation(
                &[script_fact_for_test("check", script_body)],
                "astro",
                "check",
            ),
            safely_runs_astro_build: package_script_command_parser::has_safe_tool_invocation(
                &[script_fact_for_test("build", "astro build")],
                "astro",
                "build",
            ),
            safely_runs_syncpack_lint: package_script_command_parser::has_safe_tool_invocation(
                &[script_fact_for_test("check", script_body)],
                "syncpack",
                "lint",
            ),
        },
    }
}

fn nested_parsed_package() -> G3TsAstroPackageSurfaceState {
    let mut package = parsed_package(true, true, true, true, false, true, true);
    if let G3TsAstroPackageSurfaceState::Parsed { snapshot } = &mut package {
        snapshot.rel_path = "apps/landing/package.json".to_owned();
        snapshot.package_name = Some("landing".to_owned());
    }
    package
}

fn script_commands_for_test(
    script_name: &str,
    script_body: &str,
) -> Vec<G3TsAstroPackageScriptCommand> {
    script_fact_for_test(script_name, script_body)
        .commands
        .iter()
        .map(|command| convert_script_command(script_name, command))
        .collect()
}

fn script_tool_invocations_for_test(
    script_name: &str,
    script_body: &str,
) -> Vec<G3TsAstroPackageScriptToolInvocation> {
    script_fact_for_test(script_name, script_body)
        .tool_invocations
        .iter()
        .map(convert_script_tool_invocation)
        .collect()
}

fn convert_script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsAstroPackageScriptToolInvocation {
    G3TsAstroPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        command_index: invocation.command_index,
        invocation: invocation.invocation.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(convert_script_separator),
        followed_by: invocation.followed_by.map(convert_script_separator),
    }
}

fn script_parse_blockers_for_test(
    script_name: &str,
    script_body: &str,
) -> Vec<G3TsAstroPackageScriptParseBlocker> {
    let fact = script_fact_for_test(script_name, script_body);
    convert_script_parse_blocker(&fact).into_iter().collect()
}

fn script_fact_for_test(script_name: &str, script_body: &str) -> PackageScriptParseFact {
    package_script_command_parser::parse(script_name, script_body)
        .expect("test package script should parse")
}

fn convert_script_command(
    script_name: &str,
    command: &PackageScriptCommand,
) -> G3TsAstroPackageScriptCommand {
    G3TsAstroPackageScriptCommand {
        script_name: script_name.to_owned(),
        invocation: command.invocation.clone(),
        executable: command.executable.clone(),
        args: command.args.clone(),
        preceded_by: command.preceded_by.map(convert_script_separator),
    }
}

fn convert_script_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsAstroPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsAstroPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsAstroPackageScriptCommandSeparator::Or,
    }
}

fn convert_script_parse_blocker(
    fact: &PackageScriptParseFact,
) -> Option<G3TsAstroPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => {
            Some(G3TsAstroPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}

fn syncpack_config() -> G3TsAstroSyncpackConfigState {
    syncpack_config_with_groups(required_syncpack_version_groups())
}

fn syncpack_config_with_groups(
    version_groups: Vec<TestSyncpackVersionGroup>,
) -> G3TsAstroSyncpackConfigState {
    syncpack_config_with_source_and_groups(vec!["package.json".to_owned()], version_groups)
}

fn syncpack_config_with_source_and_groups(
    source: Vec<String>,
    version_groups: Vec<TestSyncpackVersionGroup>,
) -> G3TsAstroSyncpackConfigState {
    syncpack_config_at_with_source_and_groups(".syncpackrc", source, version_groups)
}

fn syncpack_config_at_with_source_and_groups(
    rel_path: &str,
    source: Vec<String>,
    version_groups: Vec<TestSyncpackVersionGroup>,
) -> G3TsAstroSyncpackConfigState {
    syncpack_config_for_package_at_with_source_and_groups(
        rel_path,
        "package.json",
        Some("landing"),
        source,
        version_groups,
    )
}

fn syncpack_config_for_package_at_with_source_and_groups(
    rel_path: &str,
    package_rel_path: &str,
    _package_name: Option<&str>,
    source: Vec<String>,
    version_groups: Vec<TestSyncpackVersionGroup>,
) -> G3TsAstroSyncpackConfigState {
    let source_covers_package_manifest =
        syncpack_source_covers_package(&source, rel_path, package_rel_path);
    let missing_required_stack_pins = required_syncpack_pins()
        .into_iter()
        .filter(|pin| {
            !has_one_canonical_pin_group(
                &version_groups,
                &pin.dependency,
                &pin.version,
                &["prod", "dev"],
            )
        })
        .collect::<Vec<_>>();
    let missing_forbidden_bans = FORBIDDEN_SYNCPACK_DEPS
        .into_iter()
        .filter(|dependency| {
            !has_one_canonical_ban_group(
                &version_groups,
                dependency,
                &["prod", "dev", "optional", "peer"],
            )
        })
        .map(str::to_owned)
        .collect();

    G3TsAstroSyncpackConfigState::Parsed {
        snapshot: G3TsAstroSyncpackConfigSnapshot {
            rel_path: rel_path.to_owned(),
            source_covers_package_manifest,
            missing_required_stack_pins,
            missing_forbidden_bans,
        },
    }
}

fn syncpack_source_covers_package(
    source: &[String],
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> bool {
    source.len() == 1
        && source.first().is_some_and(|entry| entry == "package.json")
        && syncpack_config_is_app_local(syncpack_rel_path, package_rel_path)
}

fn has_one_canonical_pin_group(
    version_groups: &[TestSyncpackVersionGroup],
    dependency: &str,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_pin_group(group, version, dependency_types)
}

fn has_one_canonical_ban_group(
    version_groups: &[TestSyncpackVersionGroup],
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_ban_group(group, dependency_types)
}

fn group_targets_dependency(group: &TestSyncpackVersionGroup, dependency: &str) -> bool {
    string_sets_match_exactly(&group.dependencies, &[dependency])
}

fn canonical_pin_group(
    group: &TestSyncpackVersionGroup,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    group.packages.is_empty()
        && group.specifier_types.is_empty()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && !group.is_ignored
        && !group.is_banned
        && group.pin_version.as_deref() == Some(version)
}

fn canonical_ban_group(group: &TestSyncpackVersionGroup, dependency_types: &[&str]) -> bool {
    group.packages.is_empty()
        && group.specifier_types.is_empty()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && !group.is_ignored
        && group.is_banned
        && group.pin_version.is_none()
}

fn string_sets_match_exactly(left: &[String], right: &[&str]) -> bool {
    left.len() == right.len()
        && BTreeSet::from_iter(left.iter().map(String::as_str))
            == BTreeSet::from_iter(right.iter().copied())
}

fn syncpack_config_is_app_local(syncpack_rel_path: &str, package_rel_path: &str) -> bool {
    let expected_rel_path = package_rel_path.strip_suffix("/package.json").map_or_else(
        || ".syncpackrc".to_owned(),
        |app_root| format!("{app_root}/.syncpackrc"),
    );

    syncpack_rel_path == expected_rel_path
}

fn required_syncpack_pins() -> Vec<G3TsAstroSyncpackRequiredPin> {
    [
        ("astro", "6.1.9"),
        ("@astrojs/react", "5.0.4"),
        ("@astrojs/mdx", "5.0.4"),
        ("@astrojs/check", "0.9.8"),
        ("@astrojs/sitemap", "3.7.2"),
        ("astro-robots", "2.3.1"),
        ("@nuasite/checks", "0.18.0"),
        ("g3ts-astro-nuasite-checks", "0.1.0"),
        ("schema-dts", "2.0.0"),
        ("react", "19.2.5"),
        ("react-dom", "19.2.5"),
        ("@types/react", "19.2.14"),
        ("@types/react-dom", "19.2.3"),
        ("typescript", "5.9.3"),
        ("eslint-plugin-astro", "1.7.0"),
        ("g3ts-eslint-plugin-astro-pipeline", "0.1.5"),
        ("eslint-plugin-i18next", "6.1.4"),
        ("eslint-plugin-mdx", "3.7.0"),
    ]
    .into_iter()
    .map(|(dependency, version)| G3TsAstroSyncpackRequiredPin {
        dependency: dependency.to_owned(),
        version: version.to_owned(),
    })
    .collect()
}

fn forbidden_syncpack_deps() -> Vec<String> {
    FORBIDDEN_SYNCPACK_DEPS
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn required_syncpack_version_groups() -> Vec<TestSyncpackVersionGroup> {
    let mut groups = [
        ("astro", "6.1.9"),
        ("@astrojs/react", "5.0.4"),
        ("@astrojs/mdx", "5.0.4"),
        ("@astrojs/check", "0.9.8"),
        ("@astrojs/sitemap", "3.7.2"),
        ("astro-robots", "2.3.1"),
        ("@nuasite/checks", "0.18.0"),
        ("g3ts-astro-nuasite-checks", "0.1.0"),
        ("schema-dts", "2.0.0"),
        ("react", "19.2.5"),
        ("react-dom", "19.2.5"),
        ("@types/react", "19.2.14"),
        ("@types/react-dom", "19.2.3"),
        ("typescript", "5.9.3"),
        ("eslint-plugin-astro", "1.7.0"),
        ("g3ts-eslint-plugin-astro-pipeline", "0.1.5"),
        ("eslint-plugin-i18next", "6.1.4"),
        ("eslint-plugin-mdx", "3.7.0"),
    ]
    .into_iter()
    .map(|(dependency, version)| TestSyncpackVersionGroup {
        dependencies: vec![dependency.to_owned()],
        dependency_types: vec!["prod".to_owned(), "dev".to_owned()],
        packages: Vec::new(),
        specifier_types: Vec::new(),
        pin_version: Some(version.to_owned()),
        is_banned: false,
        is_ignored: false,
    })
    .collect::<Vec<_>>();

    groups.extend(
        FORBIDDEN_SYNCPACK_DEPS
            .into_iter()
            .map(|dependency| TestSyncpackVersionGroup {
                dependencies: vec![dependency.to_owned()],
                dependency_types: vec![
                    "prod".to_owned(),
                    "dev".to_owned(),
                    "optional".to_owned(),
                    "peer".to_owned(),
                ],
                packages: Vec::new(),
                specifier_types: Vec::new(),
                pin_version: None,
                is_banned: true,
                is_ignored: false,
            }),
    );

    groups
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
    let mdx_content_plugins = vec!["mdx".to_owned()];
    if has_astro_plugin {
        astro_source_plugins.push("astro".to_owned());
        ts_source_plugins.push("astro".to_owned());
        tsx_source_plugins.push("astro".to_owned());
    }
    if pipeline_contract.astro.has_plugin {
        astro_source_plugins.push("astro-pipeline".to_owned());
    }
    if pipeline_contract.astro.has_inline_public_content_plugin {
        astro_source_plugins.push("i18next".to_owned());
    }
    if pipeline_contract.ts.has_plugin {
        ts_source_plugins.push("astro-pipeline".to_owned());
    }
    if pipeline_contract.ts.has_inline_public_content_plugin {
        ts_source_plugins.push("i18next".to_owned());
    }
    if pipeline_contract.tsx.has_plugin {
        tsx_source_plugins.push("astro-pipeline".to_owned());
    }
    if pipeline_contract.tsx.has_inline_public_content_plugin {
        tsx_source_plugins.push("i18next".to_owned());
    }
    let astro_source_error_rules = pipeline_contract.astro.error_rules();
    let ts_source_error_rules = pipeline_contract.ts.error_rules();
    let tsx_source_error_rules = pipeline_contract.tsx.error_rules();
    let mdx_content_error_rules = vec!["mdx/remark".to_owned()];
    let mut astro_source_plugin_meta_names = plugin_meta_names(&pipeline_contract.astro);
    let mut ts_source_plugin_meta_names = plugin_meta_names(&pipeline_contract.ts);
    let mut tsx_source_plugin_meta_names = plugin_meta_names(&pipeline_contract.tsx);
    let mut astro_source_plugin_package_names = plugin_package_names(&pipeline_contract.astro);
    let mut ts_source_plugin_package_names = plugin_package_names(&pipeline_contract.ts);
    let mut tsx_source_plugin_package_names = plugin_package_names(&pipeline_contract.tsx);
    if has_astro_plugin {
        let _previous = astro_source_plugin_meta_names
            .insert("astro".to_owned(), "eslint-plugin-astro".to_owned());
        let _previous = ts_source_plugin_meta_names
            .insert("astro".to_owned(), "eslint-plugin-astro".to_owned());
        let _previous = tsx_source_plugin_meta_names
            .insert("astro".to_owned(), "eslint-plugin-astro".to_owned());
        let _previous = astro_source_plugin_package_names
            .insert("astro".to_owned(), vec!["eslint-plugin-astro".to_owned()]);
        let _previous = ts_source_plugin_package_names
            .insert("astro".to_owned(), vec!["eslint-plugin-astro".to_owned()]);
        let _previous = tsx_source_plugin_package_names
            .insert("astro".to_owned(), vec!["eslint-plugin-astro".to_owned()]);
    }

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            astro_source_probe_present: true,
            ts_source_probe_present: true,
            tsx_source_probe_present: true,
            mdx_content_probe_present: true,
            astro_source_plugins,
            ts_source_plugins,
            tsx_source_plugins,
            mdx_content_plugins,
            astro_source_plugin_meta_names,
            ts_source_plugin_meta_names,
            tsx_source_plugin_meta_names,
            mdx_content_plugin_meta_names: BTreeMap::from([(
                "mdx".to_owned(),
                "eslint-plugin-mdx".to_owned(),
            )]),
            astro_source_plugin_package_names,
            ts_source_plugin_package_names,
            tsx_source_plugin_package_names,
            mdx_content_plugin_package_names: BTreeMap::from([(
                "mdx".to_owned(),
                vec!["eslint-plugin-mdx".to_owned()],
            )]),
            astro_source_error_rules,
            ts_source_error_rules,
            tsx_source_error_rules,
            mdx_content_error_rules,
            astro_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .astro
                .effective_route_scoped_rules(),
            ts_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .ts
                .effective_route_scoped_rules(),
            tsx_source_effective_route_scoped_pipeline_rules: pipeline_contract
                .tsx
                .effective_route_scoped_rules(),
            astro_source_route_scoped_pipeline_rule_scopes: pipeline_contract
                .astro
                .route_scoped_rule_scopes(),
            ts_source_route_scoped_pipeline_rule_scopes: pipeline_contract
                .ts
                .route_scoped_rule_scopes(),
            tsx_source_route_scoped_pipeline_rule_scopes: pipeline_contract
                .tsx
                .route_scoped_rule_scopes(),
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
            astro_source_effective_inline_public_content_rules: pipeline_contract
                .astro
                .effective_inline_public_content_rules(),
            ts_source_effective_inline_public_content_rules: pipeline_contract
                .ts
                .effective_inline_public_content_rules(),
            tsx_source_effective_inline_public_content_rules: pipeline_contract
                .tsx
                .effective_inline_public_content_rules(),
            astro_source_probe_ignored: false,
            ts_source_probe_ignored: false,
            tsx_source_probe_ignored: false,
            mdx_content_probe_ignored: false,
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
    has_inline_public_content_plugin: bool,
    has_inline_public_content_policy: bool,
}

fn plugin_meta_names(lane: &PipelineLaneState) -> BTreeMap<String, String> {
    let mut names = BTreeMap::new();
    if lane.has_plugin {
        let _previous = names.insert(
            "astro-pipeline".to_owned(),
            "g3ts-eslint-plugin-astro-pipeline".to_owned(),
        );
    }
    if lane.has_inline_public_content_plugin {
        let _previous = names.insert("i18next".to_owned(), "eslint-plugin-i18next".to_owned());
    }
    names
}

fn plugin_package_names(lane: &PipelineLaneState) -> BTreeMap<String, Vec<String>> {
    let mut names = BTreeMap::new();
    if lane.has_plugin {
        let _previous = names.insert(
            "astro-pipeline".to_owned(),
            vec!["g3ts-eslint-plugin-astro-pipeline".to_owned()],
        );
    }
    if lane.has_inline_public_content_plugin {
        let _previous = names.insert(
            "i18next".to_owned(),
            vec!["eslint-plugin-i18next".to_owned()],
        );
    }
    names
}

impl PipelineLaneState {
    const fn disabled() -> Self {
        Self {
            has_plugin: false,
            has_required_rules: false,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
            has_inline_public_content_plugin: false,
            has_inline_public_content_policy: false,
        }
    }

    const fn plugin_only() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: false,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: false,
        }
    }

    const fn rules_without_scope_options() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    const fn rules_with_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: true,
            has_content_source_scope: true,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    const fn rules_with_endpoint_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Endpoint,
            has_content_data_scope: true,
            has_content_source_scope: true,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    const fn rules_with_scope_but_without_content_data_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: false,
            has_content_source_scope: true,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    const fn rules_with_scope_but_without_content_source_scope() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: true,
            has_content_source_scope: false,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    const fn rules_with_scope_but_without_inline_public_content_policy() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::Route,
            has_content_data_scope: true,
            has_content_source_scope: true,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: false,
        }
    }

    const fn rules_without_effective_route_coverage() -> Self {
        Self {
            has_plugin: true,
            has_required_rules: true,
            scope_kind: ScopeKind::None,
            has_content_data_scope: false,
            has_content_source_scope: false,
            has_inline_public_content_plugin: true,
            has_inline_public_content_policy: true,
        }
    }

    fn error_rules(self) -> Vec<String> {
        if !self.has_required_rules {
            return Vec::new();
        }

        let mut rules = vec![
            "astro-pipeline/no-authored-content-fs-read".to_owned(),
            "astro-pipeline/no-authored-content-glob".to_owned(),
            "astro-pipeline/no-authored-content-imports".to_owned(),
            "astro-pipeline/no-content-data-modules-in-routes".to_owned(),
            "astro-pipeline/no-direct-astro-content-in-routes".to_owned(),
            "astro-pipeline/no-runtime-mdx-eval".to_owned(),
            "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
            "astro-pipeline/no-side-loader-imports".to_owned(),
            "astro-pipeline/no-velite-imports".to_owned(),
        ];
        if self.has_inline_public_content_policy {
            rules.push("i18next/no-literal-string".to_owned());
        }
        rules
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
            "astro-pipeline/require-approved-content-adapter-in-routes".to_owned(),
            "astro-pipeline/no-side-loader-imports".to_owned(),
            "astro-pipeline/no-velite-imports".to_owned(),
        ]
    }

    fn route_scoped_rule_scopes(self) -> Vec<G3TsAstroPipelineRuleScopeSnapshot> {
        if !self.has_required_rules {
            return Vec::new();
        }

        let route_globs = match self.scope_kind {
            ScopeKind::None => Vec::new(),
            ScopeKind::Route => vec!["src/pages/**/*.astro".to_owned()],
            ScopeKind::Endpoint => vec!["src/pages/**/*.json.astro".to_owned()],
        };
        let endpoint_globs = match self.scope_kind {
            ScopeKind::None => Vec::new(),
            ScopeKind::Route => vec!["src/pages/**/*.ts".to_owned()],
            ScopeKind::Endpoint => vec!["src/pages/**/*.json.ts".to_owned()],
        };

        self.effective_route_scoped_rules()
            .into_iter()
            .map(|rule_name| G3TsAstroPipelineRuleScopeSnapshot {
                rule_name,
                route_globs: route_globs.clone(),
                endpoint_globs: endpoint_globs.clone(),
            })
            .collect()
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

    fn effective_inline_public_content_rules(self) -> Vec<String> {
        if !self.has_inline_public_content_policy {
            return Vec::new();
        }

        vec!["i18next/no-literal-string".to_owned()]
    }
}

#[derive(Clone, Copy)]
enum ScopeKind {
    None,
    Route,
    Endpoint,
}
