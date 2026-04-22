use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::PathBuf;

use cargo_toml_parser::types::CargoToml;
use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;
use g3rs_release_types as release_types;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::{crate_base, deps, paths, repo, root};
use g3rs_release_types::G3RsReleaseWorkflowAnalysis as WorkflowAnalysis;

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
pub(super) struct CrateBase {
    pub(super) name: String,
    pub(super) cargo_rel_path: String,
    pub(super) cargo_abs_path: PathBuf,
    pub(super) cargo: CargoToml,
    pub(super) publishable: bool,
    pub(super) is_binary: bool,
    pub(super) is_library: bool,
    pub(super) binary_target_names: BTreeSet<String>,
    pub(super) readme: CrateReadmeFacts,
}

pub(crate) fn config_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseConfigChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(config_input(crawl))
}

pub(super) fn config_input(
    crawl: &G3RsWorkspaceCrawl,
) -> release_types::G3RsReleaseConfigChecksInput {
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

pub(super) fn source_input(
    crawl: &G3RsWorkspaceCrawl,
) -> release_types::G3RsReleaseSourceChecksInput {
    collect(crawl, std::env::var_os("PATH").as_deref()).source
}

pub(crate) fn filetree_result(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<release_types::G3RsReleaseFileTreeChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(filetree_input(crawl))
}

pub(super) fn filetree_input(
    crawl: &G3RsWorkspaceCrawl,
) -> release_types::G3RsReleaseFileTreeChecksInput {
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

    let publishable_count = crate_bases.iter().filter(|krate| krate.publishable).count();
    let (release_plz_exists, release_plz_rel_path, release_plz, _) =
        repo::parse_release_plz(crawl, &mut config_failures);
    let (cliff_exists, cliff_rel_path, cliff) = repo::parse_cliff(crawl, &mut config_failures);
    let workflows = repo::collect_workflows(crawl, &mut config_failures);

    let repo_config = release_types::G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: root_cargo
            .as_ref()
            .map(|root| root.cargo.clone())
            .unwrap_or_else(default_root_cargo),
        release_plz_rel_path: release_plz_rel_path.clone(),
        release_plz_exists,
        release_plz,
        cliff_rel_path: cliff_rel_path.clone(),
        cliff_exists,
        cliff,
        workflows: workflows
            .iter()
            .map(|workflow| release_types::G3RsReleaseWorkflow {
                rel_path: workflow.rel_path.clone(),
                analysis: workflow.analysis.clone(),
            })
            .collect(),
        semver_checks_installed: repo::tool_is_available("cargo-semver-checks", path_env),
    };

    let config_crates = crate_bases
        .iter()
        .map(|krate| release_types::G3RsReleaseConfigCrate {
            name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            cargo: krate.cargo.clone(),
            workspace_package: root_workspace_package.clone(),
            is_binary: krate.is_binary,
            is_library: krate.is_library,
            binary_target_names: krate.binary_target_names.clone(),
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
            .map(|edge| {
                let source = config_crates
                    .iter()
                    .find(|candidate| candidate.cargo_rel_path == krate.cargo_rel_path)
                    .expect("source crate should exist")
                    .clone();
                let target = config_crates
                    .iter()
                    .find(|candidate| candidate.name == edge.dep_package_name)
                    .cloned();
                release_types::G3RsReleaseConfigEdge {
                    source,
                    dep_name: edge.dep_name,
                    dep_package_name: edge.dep_package_name,
                    section_label: edge.section_label,
                    target_label: edge.target_label,
                    has_path: edge.has_path,
                    path_target_kind: edge.path_target_kind,
                    version_req: edge.version_req,
                    target,
                }
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
            repo_checks: vec![repo_config],
            crate_checks: config_crates,
            edge_checks: edges,
            input_failure_checks: config_failures,
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

fn default_root_cargo() -> CargoToml {
    cargo_toml_parser::parse(
        r#"
[workspace]
members = []
resolver = "2"
"#,
    )
    .expect("default root cargo should parse")
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
