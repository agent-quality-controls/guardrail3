use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use cargo_toml_parser::types::CargoToml;
use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;
use g3rs_release_types as release_types;

use super::{crate_base, deps, paths, repo, root, workflow_predicates};
use g3rs_release_types::G3RsReleaseWorkflowAnalysis as WorkflowAnalysis;

#[derive(Debug, Clone)]
/// `CollectedRelease` value.
pub(super) struct CollectedRelease {
    /// `config` field.
    pub(super) config: release_types::G3RsReleaseConfigChecksInput,
    /// `filetree` field.
    pub(super) filetree: release_types::G3RsReleaseFileTreeChecksInput,
    /// `source` field.
    pub(super) source: release_types::G3RsReleaseSourceChecksInput,
}

#[derive(Debug, Clone)]
/// `RootCargo` value.
pub(super) struct RootCargo {
    /// `cargo` field.
    pub(super) cargo: CargoToml,
    /// `cargo_abs_path` field.
    pub(super) cargo_abs_path: PathBuf,
}

#[derive(Debug, Clone)]
/// `ParsedCrate` value.
pub(super) struct ParsedCrate {
    /// `rel_dir` field.
    pub(super) rel_dir: String,
    /// `cargo_rel_path` field.
    pub(super) cargo_rel_path: String,
    /// `cargo_abs_path` field.
    pub(super) cargo_abs_path: PathBuf,
    /// `cargo` field.
    pub(super) cargo: CargoToml,
}

#[derive(Debug, Clone)]
/// `WorkflowFacts` value.
pub(super) struct WorkflowFacts {
    /// `rel_path` field.
    pub(super) rel_path: String,
    /// `analysis` field.
    pub(super) analysis: WorkflowAnalysis,
}

#[derive(Debug, Clone)]
/// `CrateReadmeFacts` value.
pub(super) struct CrateReadmeFacts {
    /// `declared_false` field.
    pub(super) declared_false: bool,
    /// `rel_path` field.
    pub(super) rel_path: String,
    /// `abs_path` field.
    pub(super) abs_path: PathBuf,
    /// `exists` field.
    pub(super) exists: bool,
}

#[derive(Debug, Clone)]
/// `CrateBase` value.
pub(super) struct CrateBase {
    /// `name` field.
    pub(super) name: String,
    /// `cargo_rel_path` field.
    pub(super) cargo_rel_path: String,
    /// `cargo_abs_path` field.
    pub(super) cargo_abs_path: PathBuf,
    /// `cargo` field.
    pub(super) cargo: CargoToml,
    /// `publishable` field.
    pub(super) publishable: bool,
    /// `is_binary` field.
    pub(super) is_binary: bool,
    /// `is_library` field.
    pub(super) is_library: bool,
    /// `binary_target_names` field.
    pub(super) binary_target_names: BTreeSet<String>,
    /// `readme` field.
    pub(super) readme: CrateReadmeFacts,
}

/// Read the host `PATH` environment variable through the centralized boundary.
#[expect(
    clippy::disallowed_methods,
    reason = "this is the centralized env::var_os boundary for release ingestion PATH lookups"
)]
fn current_path_env() -> Option<OsString> {
    std::env::var_os("PATH")
}

/// `config_result` function.
pub(crate) fn config_result(
    crawl: &G3WorkspaceCrawl,
) -> Result<release_types::G3RsReleaseConfigChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(config_input(crawl))
}

/// `config_input` function.
pub(super) fn config_input(
    crawl: &G3WorkspaceCrawl,
) -> release_types::G3RsReleaseConfigChecksInput {
    config_input_with_path(crawl, current_path_env().as_deref())
}

/// `config_input_with_path` function.
pub(super) fn config_input_with_path(
    crawl: &G3WorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> release_types::G3RsReleaseConfigChecksInput {
    collect(crawl, path_env).config
}

/// `source_result` function.
pub(crate) fn source_result(
    crawl: &G3WorkspaceCrawl,
) -> Result<release_types::G3RsReleaseSourceChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(source_input(crawl))
}

/// `source_input` function.
pub(super) fn source_input(
    crawl: &G3WorkspaceCrawl,
) -> release_types::G3RsReleaseSourceChecksInput {
    collect(crawl, current_path_env().as_deref()).source
}

/// `filetree_result` function.
pub(crate) fn filetree_result(
    crawl: &G3WorkspaceCrawl,
) -> Result<release_types::G3RsReleaseFileTreeChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(filetree_input(crawl))
}

/// `filetree_input` function.
pub(super) fn filetree_input(
    crawl: &G3WorkspaceCrawl,
) -> release_types::G3RsReleaseFileTreeChecksInput {
    collect(crawl, current_path_env().as_deref()).filetree
}

/// `repo_root_result` function.
pub(crate) fn repo_root_result(
    crawl: &G3WorkspaceCrawl,
) -> Result<release_types::G3RsReleaseConfigRepo, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    let collected = collect(crawl, current_path_env().as_deref());
    collected
        .config
        .repos
        .into_iter()
        .next()
        .ok_or(IngestionError::CargoTomlNotFound)
}

/// `require_pointed_workspace_root` function.
fn require_pointed_workspace_root(crawl: &G3WorkspaceCrawl) -> Result<(), IngestionError> {
    let Some(entry) = g3_workspace_crawl::entry(crawl, "Cargo.toml") else {
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

/// `collect` function.
pub(super) fn collect(crawl: &G3WorkspaceCrawl, path_env: Option<&OsStr>) -> CollectedRelease {
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

    let repo_config = build_repo_config(
        crawl,
        root_cargo.as_ref(),
        release_plz_exists,
        &release_plz_rel_path,
        release_plz,
        cliff_exists,
        &cliff_rel_path,
        cliff,
        &workflows,
        path_env,
    );

    let config_crates = build_config_crates(&crate_bases, root_workspace_package.as_ref());

    let workspace_dependencies = root_cargo
        .as_ref()
        .and_then(|root| root.cargo.workspace.as_ref())
        .map(|workspace| workspace.dependencies.clone())
        .unwrap_or_default();

    let edges = build_edges(crawl, &crate_bases, &config_crates, &workspace_dependencies);

    let repo_filetree = build_repo_filetree(
        crawl,
        publishable_count,
        release_plz_exists,
        release_plz_rel_path,
        cliff_exists,
        cliff_rel_path,
    );

    let filetree_readmes = build_filetree_readmes(&crate_bases);

    let source_readmes = build_source_readmes(crawl, &crate_bases, &mut source_failures);

    CollectedRelease {
        config: release_types::G3RsReleaseConfigChecksInput {
            repos: vec![repo_config],
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

/// Build the repo-level config payload, including workflow flags and tool availability.
#[expect(
    clippy::too_many_arguments,
    reason = "repo config aggregates several independent file-derived fields"
)]
fn build_repo_config(
    crawl: &G3WorkspaceCrawl,
    root_cargo: Option<&RootCargo>,
    release_plz_exists: bool,
    release_plz_rel_path: &str,
    release_plz: Option<release_plz_toml_parser::types::ReleasePlzToml>,
    cliff_exists: bool,
    cliff_rel_path: &str,
    cliff: Option<cliff_toml_parser::types::CliffToml>,
    workflows: &[WorkflowFacts],
    path_env: Option<&OsStr>,
) -> release_types::G3RsReleaseConfigRepo {
    let has_release_plz_workflow = workflows
        .iter()
        .any(workflow_predicates::workflow_has_release_plz);
    let release_plz_workflow_rel_path = workflows
        .iter()
        .find(|workflow| workflow_predicates::workflow_has_release_plz(workflow))
        .map(|workflow| workflow.rel_path.clone());
    let has_publish_dry_run_workflow = workflows
        .iter()
        .any(workflow_predicates::workflow_has_publish_dry_run);
    let publish_dry_run_workflow_rel_path = workflows
        .iter()
        .find(|workflow| workflow_predicates::workflow_has_publish_dry_run(workflow))
        .map(|workflow| workflow.rel_path.clone());
    let has_registry_token_workflow = workflows
        .iter()
        .any(workflow_predicates::workflow_has_registry_token);
    let registry_token_workflow_rel_path = workflows
        .iter()
        .find(|workflow| workflow_predicates::workflow_has_registry_token(workflow))
        .map(|workflow| workflow.rel_path.clone());

    let _ = crawl;
    release_types::G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: root_cargo.map_or_else(default_root_cargo, |root| root.cargo.clone()),
        release_plz_rel_path: release_plz_rel_path.to_owned(),
        release_plz_exists,
        release_plz,
        cliff_rel_path: cliff_rel_path.to_owned(),
        cliff_exists,
        cliff,
        workflows: workflows
            .iter()
            .map(|workflow| release_types::G3RsReleaseWorkflow {
                rel_path: workflow.rel_path.clone(),
                analysis: workflow.analysis.clone(),
            })
            .collect(),
        workflow_flags: release_types::G3RsReleaseConfigRepoWorkflowFlags {
            has_release_plz_workflow,
            has_publish_dry_run_workflow,
            has_registry_token_workflow,
        },
        release_plz_workflow_rel_path,
        publish_dry_run_workflow_rel_path,
        registry_token_workflow_rel_path,
        semver_checks_installed: repo::tool_is_available("cargo-semver-checks", path_env),
    }
}

/// Build per-crate config payloads.
fn build_config_crates(
    crate_bases: &[CrateBase],
    workspace_package: Option<&cargo_toml_parser::types::WorkspacePackageSection>,
) -> Vec<release_types::G3RsReleaseConfigCrate> {
    crate_bases
        .iter()
        .map(|krate| release_types::G3RsReleaseConfigCrate {
            name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            cargo: krate.cargo.clone(),
            workspace_package: workspace_package.cloned(),
            is_binary: krate.is_binary,
            is_library: krate.is_library,
            binary_target_names: krate.binary_target_names.clone(),
            dry_run: krate
                .publishable
                .then(|| repo::run_publish_dry_run(&krate.cargo_abs_path)),
        })
        .collect()
}

/// Build dependency edges across all parsed crates.
fn build_edges(
    crawl: &G3WorkspaceCrawl,
    crate_bases: &[CrateBase],
    config_crates: &[release_types::G3RsReleaseConfigCrate],
    workspace_dependencies: &std::collections::BTreeMap<
        String,
        cargo_toml_parser::types::Dependency,
    >,
) -> Vec<release_types::G3RsReleaseConfigEdge> {
    let mut out = Vec::new();
    for krate in crate_bases {
        for edge in deps::dependency_edges(
            crawl,
            &krate.cargo_abs_path,
            &krate.cargo,
            workspace_dependencies,
        ) {
            let Some(source) = config_crates
                .iter()
                .find(|candidate| candidate.cargo_rel_path == krate.cargo_rel_path)
                .cloned()
            else {
                continue;
            };
            let target = config_crates
                .iter()
                .find(|candidate| candidate.name == edge.dep_package_name)
                .cloned();
            out.push(release_types::G3RsReleaseConfigEdge {
                source,
                dep_name: edge.dep_name,
                dep_package_name: edge.dep_package_name,
                section_label: edge.section_label,
                target_label: edge.target_label,
                has_path: edge.has_path,
                path_target_kind: edge.path_target_kind,
                version_req: edge.version_req,
                target,
            });
        }
    }
    out
}

/// Build source-checks READMEs by reading each publishable crate's README.
fn build_source_readmes(
    crawl: &G3WorkspaceCrawl,
    crate_bases: &[CrateBase],
    source_failures: &mut Vec<release_types::G3RsReleaseInputFailure>,
) -> Vec<release_types::G3RsReleaseSourceReadme> {
    let mut source_readmes = Vec::new();
    for krate in crate_bases {
        if !krate.publishable || krate.readme.declared_false || !krate.readme.exists {
            continue;
        }
        let Some(entry) = g3_workspace_crawl::entry(crawl, &krate.readme.rel_path) else {
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
    source_readmes
}

/// Build the file-tree repo payload describing release-relevant root files.
fn build_repo_filetree(
    crawl: &G3WorkspaceCrawl,
    publishable_count: usize,
    release_plz_exists: bool,
    release_plz_rel_path: String,
    cliff_exists: bool,
    cliff_rel_path: String,
) -> release_types::G3RsReleaseFileTreeRepo {
    release_types::G3RsReleaseFileTreeRepo {
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
    }
}

/// Build per-crate file-tree README payloads from the resolved crate bases.
fn build_filetree_readmes(
    crate_bases: &[CrateBase],
) -> Vec<release_types::G3RsReleaseFileTreeReadme> {
    crate_bases
        .iter()
        .map(|krate| release_types::G3RsReleaseFileTreeReadme {
            crate_name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            publishable: krate.publishable,
            readme_declared_false: krate.readme.declared_false,
            readme_rel_path: krate.readme.rel_path.clone(),
            readme_exists: krate.readme.exists,
        })
        .collect()
}

/// `default_root_cargo` function.
///
/// # Panics
///
/// Panics if the embedded default workspace manifest fails to parse, which would indicate a build-time bug.
#[expect(
    clippy::panic,
    reason = "embedded default workspace manifest parse failure is a build-time invariant violation"
)]
fn default_root_cargo() -> CargoToml {
    let raw = r#"
[workspace]
members = []
resolver = "2"
"#;
    cargo_toml_parser::parse(raw).unwrap_or_else(|err| {
        panic!("default root cargo should parse: {err}");
    })
}

/// `input_failure` function.
pub(super) fn input_failure(
    rel_path: impl Into<String>,
    message: impl Into<String>,
) -> release_types::G3RsReleaseInputFailure {
    release_types::G3RsReleaseInputFailure {
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

/// `push_all_failures` function.
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
