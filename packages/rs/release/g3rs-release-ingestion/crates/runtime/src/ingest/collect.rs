use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::PathBuf;

use cargo_toml_parser::types::CargoToml;
use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;
use g3rs_release_types as release_types;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::workflow::WorkflowAnalysis;
use super::{crate_base, deps, paths, repo, root};

#[derive(Debug, Clone)]
pub(super) struct CollectedRelease {
    pub(super) config: release_types::G3RsReleaseConfigChecksInput,
    pub(super) filetree: release_types::G3RsReleaseFileTreeChecksInput,
    pub(super) source: release_types::G3RsReleaseSourceChecksInput,
}

#[derive(Debug, Clone)]
pub(super) struct RootCargo {
    pub(super) cargo: CargoToml,
    pub(super) cargo_abs_path: PathBuf,
    pub(super) raw: toml::Value,
}

#[derive(Debug, Clone)]
pub(super) struct ParsedCrate {
    pub(super) rel_dir: String,
    pub(super) cargo_rel_path: String,
    pub(super) cargo_abs_path: PathBuf,
    pub(super) cargo: CargoToml,
}

#[derive(Debug, Clone)]
pub(super) struct WorkflowFacts {
    pub(super) rel_path: String,
    pub(super) analysis: WorkflowAnalysis,
}

#[derive(Debug, Clone)]
pub(super) struct CrateReadmeFacts {
    pub(super) declared_false: bool,
    pub(super) rel_path: String,
    pub(super) abs_path: PathBuf,
    pub(super) exists: bool,
}

#[derive(Debug, Clone)]
pub(super) struct CrateReleaseFacts {
    pub(super) description_present: bool,
    pub(super) license_present: bool,
    pub(super) repository_present: bool,
    pub(super) keywords_count: Option<usize>,
    pub(super) categories_count: Option<usize>,
    pub(super) version_string: Option<String>,
    pub(super) workspace_version: bool,
    pub(super) version_valid: bool,
    pub(super) docs_rs_present: bool,
    pub(super) include_exclude_present: bool,
    pub(super) has_binstall_metadata: bool,
}

#[derive(Debug, Clone)]
pub(super) struct CrateBase {
    pub(super) name: String,
    pub(super) cargo_rel_path: String,
    pub(super) cargo_abs_path: PathBuf,
    pub(super) cargo: CargoToml,
    pub(super) publish_declared: bool,
    pub(super) publishable: bool,
    pub(super) is_binary: bool,
    pub(super) is_library: bool,
    pub(super) binary_target_names: BTreeSet<String>,
    pub(super) readme: CrateReadmeFacts,
    pub(super) release: CrateReleaseFacts,
}

pub(crate) fn config_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseConfigChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(config_input(crawl))
}

pub(super) fn config_input(crawl: &G3RsWorkspaceCrawl) -> release_types::G3RsReleaseConfigChecksInput {
    config_input_with_path(crawl, std::env::var_os("PATH").as_deref())
}

pub(super) fn config_input_with_path(
    crawl: &G3RsWorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> release_types::G3RsReleaseConfigChecksInput {
    collect(crawl, path_env).config
}

pub(crate) fn source_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseSourceChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(source_input(crawl))
}

pub(super) fn source_input(crawl: &G3RsWorkspaceCrawl) -> release_types::G3RsReleaseSourceChecksInput {
    collect(crawl, std::env::var_os("PATH").as_deref()).source
}

pub(crate) fn filetree_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseFileTreeChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(filetree_input(crawl))
}

pub(super) fn filetree_input(crawl: &G3RsWorkspaceCrawl) -> release_types::G3RsReleaseFileTreeChecksInput {
    collect(crawl, std::env::var_os("PATH").as_deref()).filetree
}

pub(crate) fn repo_root_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseConfigRepo, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Err(IngestionError::RepoRootChecksNotImplemented)
}

fn require_pointed_workspace_root(crawl: &G3RsWorkspaceCrawl) -> Result<(), IngestionError> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, "Cargo.toml") else {
        return Err(IngestionError::CargoTomlNotFound);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content = crate::parse::read_to_string(&entry.path.abs_path)?;
    let cargo = crate::parse::parse_cargo_toml(&content, &entry.path.abs_path)?;
    if cargo.workspace.is_none() {
        return Err(IngestionError::NormalizationFailed {
            path: entry.path.abs_path.clone(),
            reason: "root Cargo.toml must declare a [workspace] table".to_owned(),
        });
    }
    Ok(())
}

pub(super) fn collect(crawl: &G3RsWorkspaceCrawl, path_env: Option<&OsStr>) -> CollectedRelease {
    let mut config_failures = Vec::new();
    let mut filetree_failures = Vec::new();
    let mut source_failures = Vec::new();

    let root_cargo = root::parse_root_cargo(
        crawl,
        &mut config_failures,
        &mut filetree_failures,
        &mut source_failures,
    );
    let root_workspace_package = root_cargo
        .as_ref()
        .and_then(|root| root.cargo.workspace.as_ref())
        .and_then(|workspace| workspace.package.clone());
    let parsed_crates = root::collect_parsed_crates(
        crawl,
        root_cargo.as_ref(),
        &mut config_failures,
        &mut filetree_failures,
        &mut source_failures,
    );

    let crate_bases = parsed_crates
        .iter()
        .map(|krate| crate_base::build_crate_base(crawl, krate, root_workspace_package.as_ref()))
        .collect::<Vec<_>>();

    let publishable_names = crate_bases
        .iter()
        .filter(|krate| krate.publishable)
        .map(|krate| krate.name.clone())
        .collect::<BTreeSet<_>>();
    let publishable_binary_names = crate_bases
        .iter()
        .filter(|krate| krate.publishable && krate.is_binary)
        .map(|krate| krate.name.clone())
        .collect::<BTreeSet<_>>();
    let publishable_count = crate_bases.iter().filter(|krate| krate.publishable).count();
    let non_publishable_count = crate_bases.len().saturating_sub(publishable_count);
    let version_map = crate_bases
        .iter()
        .filter(|krate| krate.publishable)
        .filter_map(|krate| {
            krate
                .release
                .version_string
                .clone()
                .map(|version| (krate.name.clone(), version))
        })
        .collect::<BTreeMap<_, _>>();

    let (release_plz_exists, release_plz_rel_path, release_plz, release_plz_package_names) =
        repo::parse_release_plz(crawl, &mut config_failures);
    let (cliff_exists, cliff_rel_path, cliff) = repo::parse_cliff(crawl, &mut config_failures);
    let workflows = repo::collect_workflows(crawl, &mut config_failures);

    let repo_config = release_types::G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        release_plz_rel_path: release_plz_rel_path.clone(),
        release_plz_exists,
        release_plz,
        release_plz_package_names,
        cliff_rel_path: cliff_rel_path.clone(),
        cliff_exists,
        cliff,
        has_release_plz_workflow: workflows
            .iter()
            .any(|workflow| crate::workflow::release_plz_step_present(&workflow.analysis)),
        release_plz_workflow_rel_path: workflows
            .iter()
            .find(|workflow| crate::workflow::release_plz_step_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        has_publish_dry_run_workflow: workflows
            .iter()
            .any(|workflow| crate::workflow::publish_dry_run_step_present(&workflow.analysis)),
        publish_dry_run_workflow_rel_path: workflows
            .iter()
            .find(|workflow| crate::workflow::publish_dry_run_step_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        has_registry_token_workflow: workflows
            .iter()
            .any(|workflow| crate::workflow::registry_token_present(&workflow.analysis)),
        registry_token_workflow_rel_path: workflows
            .iter()
            .find(|workflow| crate::workflow::registry_token_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        publishable_crate_names: publishable_names.clone(),
        publishable_binary_crate_names: publishable_binary_names.clone(),
        publishable_count,
        non_publishable_count,
        semver_checks_installed: repo::tool_is_available("cargo-semver-checks", path_env),
        publish_setting: root_cargo
            .as_ref()
            .and_then(|root| repo::publish_setting_string(&root.raw)),
        release_profile_settings: root_cargo
            .as_ref()
            .map_or_else(Vec::new, |root| repo::release_profile_settings(&root.raw)),
    };

    let config_crates = crate_bases
        .iter()
        .map(|krate| release_types::G3RsReleaseConfigCrate {
            name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            cargo: krate.cargo.clone(),
            workspace_package: root_workspace_package.clone(),
            publish_declared: krate.publish_declared,
            publishable: krate.publishable,
            is_binary: krate.is_binary,
            is_library: krate.is_library,
            binary_target_names: krate.binary_target_names.clone(),
            description_present: krate.release.description_present,
            license_present: krate.release.license_present,
            repository_present: krate.release.repository_present,
            keywords_count: krate.release.keywords_count,
            categories_count: krate.release.categories_count,
            version_string: krate.release.version_string.clone(),
            workspace_version: krate.release.workspace_version,
            version_valid: krate.release.version_valid,
            docs_rs_present: krate.release.docs_rs_present,
            include_exclude_present: krate.release.include_exclude_present,
            has_binstall_metadata: krate.release.has_binstall_metadata,
            binary_release_workflow_present: workflows.iter().any(|workflow| {
                crate::workflow::binary_release_present(
                    &workflow.analysis,
                    &krate.name,
                    &krate.cargo_rel_path,
                    &krate.binary_target_names,
                    publishable_binary_names.len(),
                )
            }),
            linux_release_target_present: workflows.iter().any(|workflow| {
                crate::workflow::linux_target_present(
                    &workflow.analysis,
                    &krate.name,
                    &krate.cargo_rel_path,
                    &krate.binary_target_names,
                    publishable_binary_names.len(),
                )
            }),
            dry_run: krate
                .publishable
                .then(|| repo::run_publish_dry_run(&krate.cargo_abs_path)),
        })
        .collect::<Vec<_>>();

    let workspace_dependencies = root_cargo
        .as_ref()
        .and_then(|root| root.cargo.workspace.as_ref())
        .map(|workspace| workspace.dependencies.clone())
        .unwrap_or_default();

    let edges = crate_bases
        .iter()
        .flat_map(|krate| {
            deps::dependency_edges(
                crawl,
                &krate.cargo_abs_path,
                &krate.cargo,
                &workspace_dependencies,
            )
            .into_iter()
            .map(|edge| release_types::G3RsReleaseConfigEdge {
                crate_name: krate.name.clone(),
                cargo_rel_path: krate.cargo_rel_path.clone(),
                source_publishable: krate.publishable,
                dep_name: edge.dep_name,
                dep_package_name: edge.dep_package_name.clone(),
                section_label: edge.section_label,
                target_label: edge.target_label,
                has_path: edge.has_path,
                path_target_kind: edge.path_target_kind,
                dep_publishable: publishable_names.contains(&edge.dep_package_name),
                version_req: edge.version_req.clone(),
                actual_version: version_map.get(&edge.dep_package_name).cloned(),
                version_satisfied: edge
                    .version_req
                    .as_deref()
                    .zip(version_map.get(&edge.dep_package_name).map(String::as_str))
                    .map(|(req, actual)| deps::version_requirement_satisfied(actual, req)),
            })
        })
        .collect::<Vec<_>>();

    let repo_filetree = release_types::G3RsReleaseFileTreeRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        publishable_count,
        license_rel_path: ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"]
            .iter()
            .find(|name| paths::file_exists(crawl, name))
            .map(|name| (*name).to_owned()),
        release_plz_rel_path,
        release_plz_exists,
        cliff_rel_path,
        cliff_exists,
    };

    let filetree_readmes = crate_bases
        .iter()
        .map(|krate| release_types::G3RsReleaseFileTreeReadme {
            crate_name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            publishable: krate.publishable,
            readme_declared_false: krate.readme.declared_false,
            readme_rel_path: krate.readme.rel_path.clone(),
            readme_exists: krate.readme.exists,
        })
        .collect::<Vec<_>>();

    let mut source_readmes = Vec::new();
    for krate in &crate_bases {
        if !krate.publishable || krate.readme.declared_false || !krate.readme.exists {
            continue;
        }
        let Some(entry) = g3rs_workspace_crawl::entry(crawl, &krate.readme.rel_path) else {
            continue;
        };
        if !entry.readable {
            source_failures.push(input_failure(
                &krate.readme.rel_path,
                "Failed to read README for release checks: file is not readable.",
            ));
            continue;
        }
        match crate::parse::read_to_string(&krate.readme.abs_path) {
            Ok(content) => source_readmes.push(release_types::G3RsReleaseSourceReadme {
                crate_name: krate.name.clone(),
                cargo_rel_path: krate.cargo_rel_path.clone(),
                readme_rel_path: krate.readme.rel_path.clone(),
                content,
            }),
            Err(error) => source_failures.push(input_failure(
                &krate.readme.rel_path,
                format!("Failed to read README for release checks: {error}"),
            )),
        }
    }

    CollectedRelease {
        config: release_types::G3RsReleaseConfigChecksInput {
            repo: Some(repo_config),
            crates: config_crates,
            edges,
            input_failures: config_failures,
        },
        filetree: release_types::G3RsReleaseFileTreeChecksInput {
            repo: Some(repo_filetree),
            readmes: filetree_readmes,
            input_failures: filetree_failures,
        },
        source: release_types::G3RsReleaseSourceChecksInput {
            readmes: source_readmes,
            input_failures: source_failures,
        },
    }
}

pub(super) fn input_failure(
    rel_path: impl Into<String>,
    message: impl Into<String>,
) -> release_types::G3RsReleaseInputFailure {
    release_types::G3RsReleaseInputFailure {
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

pub(super) fn push_all_failures(
    config_failures: &mut Vec<release_types::G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<release_types::G3RsReleaseInputFailure>,
    source_failures: &mut Vec<release_types::G3RsReleaseInputFailure>,
    rel_path: impl Into<String>,
    message: impl Into<String>,
) {
    let rel_path = rel_path.into();
    let message = message.into();
    config_failures.push(input_failure(&rel_path, &message));
    filetree_failures.push(input_failure(&rel_path, &message));
    source_failures.push(input_failure(rel_path, message));
}

#[cfg(test)]
#[path = "collect_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod collect_tests;
