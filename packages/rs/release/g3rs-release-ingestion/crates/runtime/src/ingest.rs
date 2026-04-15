use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use cargo_toml_parser::{
    CargoToml, Dependency, InheritableValue, PackageSection, StringOrBool,
    TargetDependencyTables, VecStringOrBool, WorkspacePackageSection,
};
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate, G3RsReleaseConfigEdge,
    G3RsReleaseConfigRepo, G3RsReleaseDryRunOutcome, G3RsReleaseFileTreeChecksInput,
    G3RsReleaseFileTreeReadme, G3RsReleaseFileTreeRepo, G3RsReleaseInputFailure,
    G3RsReleasePathTargetKind, G3RsReleaseSourceChecksInput, G3RsReleaseSourceReadme,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};
use semver::{Version, VersionReq};

use crate::workflow::{
    binary_release_present, extract_workflow_analysis, linux_target_present,
    publish_dry_run_step_present, registry_token_present, release_plz_step_present,
    WorkflowAnalysis,
};

#[derive(Debug, Clone)]
pub(crate) struct CollectedRelease {
    pub(crate) config: G3RsReleaseConfigChecksInput,
    pub(crate) filetree: G3RsReleaseFileTreeChecksInput,
    pub(crate) source: G3RsReleaseSourceChecksInput,
}

#[derive(Debug, Clone)]
struct RootCargo {
    cargo: CargoToml,
    cargo_abs_path: PathBuf,
    raw: toml::Value,
}

#[derive(Debug, Clone)]
struct ParsedCrate {
    rel_dir: String,
    cargo_rel_path: String,
    cargo_abs_path: PathBuf,
    cargo: CargoToml,
}

#[derive(Debug, Clone)]
struct WorkflowFacts {
    rel_path: String,
    analysis: WorkflowAnalysis,
}

#[derive(Debug, Clone)]
struct CrateBase {
    name: String,
    cargo_rel_path: String,
    cargo_abs_path: PathBuf,
    cargo: CargoToml,
    publish_declared: bool,
    publishable: bool,
    is_binary: bool,
    is_library: bool,
    binary_target_names: BTreeSet<String>,
    description_present: bool,
    license_present: bool,
    repository_present: bool,
    readme_declared_false: bool,
    readme_rel_path: String,
    readme_abs_path: PathBuf,
    readme_exists: bool,
    keywords_count: Option<usize>,
    categories_count: Option<usize>,
    version_string: Option<String>,
    workspace_version: bool,
    version_valid: bool,
    docs_rs_present: bool,
    include_exclude_present: bool,
    has_binstall_metadata: bool,
}

pub(crate) fn collect(crawl: &G3RsWorkspaceCrawl, path_env: Option<&OsStr>) -> CollectedRelease {
    let mut config_failures = Vec::new();
    let mut filetree_failures = Vec::new();
    let mut source_failures = Vec::new();

    let root_cargo = parse_root_cargo(
        crawl,
        &mut config_failures,
        &mut filetree_failures,
        &mut source_failures,
    );
    let root_workspace_package = root_cargo
        .as_ref()
        .and_then(|root| root.cargo.workspace.as_ref())
        .and_then(|workspace| workspace.package.clone());
    let parsed_crates = collect_parsed_crates(
        crawl,
        root_cargo.as_ref(),
        &mut config_failures,
        &mut filetree_failures,
        &mut source_failures,
    );

    let crate_bases = parsed_crates
        .iter()
        .map(|krate| build_crate_base(crawl, krate, root_workspace_package.as_ref()))
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
                .version_string
                .as_ref()
                .map(|version| (krate.name.clone(), version.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    let (release_plz_exists, release_plz_rel_path, release_plz, release_plz_package_names) =
        parse_release_plz(crawl, &mut config_failures);
    let (cliff_exists, cliff_rel_path, cliff) = parse_cliff(crawl, &mut config_failures);
    let workflows = collect_workflows(crawl, &mut config_failures);

    let repo_config = G3RsReleaseConfigRepo {
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
            .any(|workflow| release_plz_step_present(&workflow.analysis)),
        release_plz_workflow_rel_path: workflows
            .iter()
            .find(|workflow| release_plz_step_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        has_publish_dry_run_workflow: workflows
            .iter()
            .any(|workflow| publish_dry_run_step_present(&workflow.analysis)),
        publish_dry_run_workflow_rel_path: workflows
            .iter()
            .find(|workflow| publish_dry_run_step_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        has_registry_token_workflow: workflows
            .iter()
            .any(|workflow| registry_token_present(&workflow.analysis)),
        registry_token_workflow_rel_path: workflows
            .iter()
            .find(|workflow| registry_token_present(&workflow.analysis))
            .map(|workflow| workflow.rel_path.clone()),
        publishable_crate_names: publishable_names.clone(),
        publishable_binary_crate_names: publishable_binary_names.clone(),
        publishable_count,
        non_publishable_count,
        semver_checks_installed: tool_is_available("cargo-semver-checks", path_env),
        publish_setting: root_cargo
            .as_ref()
            .and_then(|root| publish_setting_string(&root.raw)),
        release_profile_settings: root_cargo
            .as_ref()
            .map_or_else(Vec::new, |root| release_profile_settings(&root.raw)),
    };

    let config_crates = crate_bases
        .iter()
        .map(|krate| G3RsReleaseConfigCrate {
            name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            cargo: krate.cargo.clone(),
            workspace_package: root_workspace_package.clone(),
            publish_declared: krate.publish_declared,
            publishable: krate.publishable,
            is_binary: krate.is_binary,
            is_library: krate.is_library,
            binary_target_names: krate.binary_target_names.clone(),
            description_present: krate.description_present,
            license_present: krate.license_present,
            repository_present: krate.repository_present,
            keywords_count: krate.keywords_count,
            categories_count: krate.categories_count,
            version_string: krate.version_string.clone(),
            workspace_version: krate.workspace_version,
            version_valid: krate.version_valid,
            docs_rs_present: krate.docs_rs_present,
            include_exclude_present: krate.include_exclude_present,
            has_binstall_metadata: krate.has_binstall_metadata,
            binary_release_workflow_present: workflows.iter().any(|workflow| {
                binary_release_present(
                    &workflow.analysis,
                    &krate.name,
                    &krate.cargo_rel_path,
                    &krate.binary_target_names,
                    publishable_binary_names.len(),
                )
            }),
            linux_release_target_present: workflows.iter().any(|workflow| {
                linux_target_present(
                    &workflow.analysis,
                    &krate.name,
                    &krate.cargo_rel_path,
                    &krate.binary_target_names,
                    publishable_binary_names.len(),
                )
            }),
            dry_run: krate.publishable.then(|| run_publish_dry_run(&krate.cargo_abs_path)),
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
            dependency_edges(
                crawl,
                &krate.cargo_abs_path,
                &krate.cargo,
                &workspace_dependencies,
            )
                .into_iter()
                .map(|edge| G3RsReleaseConfigEdge {
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
                        .map(|(req, actual)| version_requirement_satisfied(actual, req)),
                })
        })
        .collect::<Vec<_>>();

    let repo_filetree = G3RsReleaseFileTreeRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        publishable_count,
        license_rel_path: ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"]
            .iter()
            .find(|name| file_exists(crawl, name))
            .map(|name| (*name).to_owned()),
        release_plz_rel_path,
        release_plz_exists,
        cliff_rel_path,
        cliff_exists,
    };

    let filetree_readmes = crate_bases
        .iter()
        .map(|krate| G3RsReleaseFileTreeReadme {
            crate_name: krate.name.clone(),
            cargo_rel_path: krate.cargo_rel_path.clone(),
            publishable: krate.publishable,
            readme_declared_false: krate.readme_declared_false,
            readme_rel_path: krate.readme_rel_path.clone(),
            readme_exists: krate.readme_exists,
        })
        .collect::<Vec<_>>();

    let mut source_readmes = Vec::new();
    for krate in &crate_bases {
        if !krate.publishable || krate.readme_declared_false || !krate.readme_exists {
            continue;
        }
        let Some(entry) = crawl.entry(&krate.readme_rel_path) else {
            continue;
        };
        if !entry.readable {
            source_failures.push(input_failure(
                &krate.readme_rel_path,
                "Failed to read README for release checks: file is not readable.",
            ));
            continue;
        }
        match crate::parse::read_to_string(&krate.readme_abs_path) {
            Ok(content) => source_readmes.push(G3RsReleaseSourceReadme {
                crate_name: krate.name.clone(),
                cargo_rel_path: krate.cargo_rel_path.clone(),
                readme_rel_path: krate.readme_rel_path.clone(),
                content,
            }),
            Err(error) => source_failures.push(input_failure(
                &krate.readme_rel_path,
                format!("Failed to read README for release checks: {error}"),
            )),
        }
    }

    CollectedRelease {
        config: G3RsReleaseConfigChecksInput {
            repo: Some(repo_config),
            crates: config_crates,
            edges,
            input_failures: config_failures,
        },
        filetree: G3RsReleaseFileTreeChecksInput {
            repo: Some(repo_filetree),
            readmes: filetree_readmes,
            input_failures: filetree_failures,
        },
        source: G3RsReleaseSourceChecksInput {
            readmes: source_readmes,
            input_failures: source_failures,
        },
    }
}

fn parse_root_cargo(
    crawl: &G3RsWorkspaceCrawl,
    config_failures: &mut Vec<G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<G3RsReleaseInputFailure>,
    source_failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Option<RootCargo> {
    let Some(entry) = crate::select::select_cargo_toml(crawl) else {
        push_all_failures(
            config_failures,
            filetree_failures,
            source_failures,
            "Cargo.toml",
            "Release workspace root is missing Cargo.toml.",
        );
        return None;
    };
    if !entry.readable {
        push_all_failures(
            config_failures,
            filetree_failures,
            source_failures,
            &entry.path.rel_path,
            "Failed to read root Cargo.toml for release checks: file is not readable.",
        );
        return None;
    }

    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                format!("Failed to read root Cargo.toml for release checks: {error}"),
            );
            return None;
        }
    };

    let cargo = match crate::parse::parse_cargo_toml(&content, &entry.path.abs_path) {
        Ok(cargo) => cargo,
        Err(error) => {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                format!("Failed to parse root Cargo.toml for release checks: {error}"),
            );
            return None;
        }
    };

    let raw = match crate::parse::parse_raw_toml(&content, &entry.path.abs_path) {
        Ok(raw) => raw,
        Err(error) => {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                format!("Failed to parse root Cargo.toml for release checks: {error}"),
            );
            return None;
        }
    };

    Some(RootCargo {
        cargo,
        cargo_abs_path: entry.path.abs_path.clone(),
        raw,
    })
}

fn collect_parsed_crates(
    crawl: &G3RsWorkspaceCrawl,
    root: Option<&RootCargo>,
    config_failures: &mut Vec<G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<G3RsReleaseInputFailure>,
    source_failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Vec<ParsedCrate> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut crates = Vec::new();
    if root.cargo.package.is_some() {
        crates.push(ParsedCrate {
            rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            cargo_abs_path: root.cargo_abs_path.clone(),
            cargo: root.cargo.clone(),
        });
    }

    let member_rels = match root.cargo.workspace.as_ref() {
        Some(_) => match crate::select::collect_member_rels(crawl, &root.cargo) {
            Ok(member_rels) => member_rels,
            Err(reason) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    "Cargo.toml",
                    format!("Failed to normalize workspace members for release checks: {reason}"),
                );
                Vec::new()
            }
        },
        None => Vec::new(),
    };

    for member_rel in member_rels {
        if member_rel.is_empty() {
            continue;
        }
        let Some(entry) = crate::select::select_member_manifest(crawl, &member_rel) else {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                crate::select::member_manifest_rel_path(&member_rel),
                format!("Declared workspace member `{member_rel}` is missing Cargo.toml."),
            );
            continue;
        };
        if !entry.readable {
            push_all_failures(
                config_failures,
                filetree_failures,
                source_failures,
                &entry.path.rel_path,
                "Failed to read member Cargo.toml for release checks: file is not readable.",
            );
            continue;
        }

        let content = match crate::parse::read_to_string(&entry.path.abs_path) {
            Ok(content) => content,
            Err(error) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    &entry.path.rel_path,
                    format!("Failed to read member Cargo.toml for release checks: {error}"),
                );
                continue;
            }
        };

        let cargo = match crate::parse::parse_cargo_toml(&content, &entry.path.abs_path) {
            Ok(cargo) => cargo,
            Err(error) => {
                push_all_failures(
                    config_failures,
                    filetree_failures,
                    source_failures,
                    &entry.path.rel_path,
                    format!("Failed to parse member Cargo.toml for release checks: {error}"),
                );
                continue;
            }
        };

        crates.push(ParsedCrate {
            rel_dir: member_rel,
            cargo_rel_path: entry.path.rel_path.clone(),
            cargo_abs_path: entry.path.abs_path.clone(),
            cargo,
        });
    }

    crates.sort_by(|left, right| left.cargo_rel_path.cmp(&right.cargo_rel_path));
    crates
}

fn build_crate_base(
    crawl: &G3RsWorkspaceCrawl,
    krate: &ParsedCrate,
    workspace_package: Option<&WorkspacePackageSection>,
) -> CrateBase {
    let package = krate.cargo.package.as_ref();
    let name = package
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| krate.cargo_rel_path.clone());
    let publish_declared = publish_declared(package);
    let publishable = publishable(package, workspace_package);
    let is_binary = is_binary_crate(crawl, &krate.rel_dir, &krate.cargo);
    let binary_target_names = binary_target_names(crawl, &krate.rel_dir, &krate.cargo);
    let readme_declared_false = readme_declared_false(package, workspace_package);
    let (readme_field, readme_from_workspace) = readme_path_field(package, workspace_package);
    let readme_base_rel_dir = if readme_from_workspace {
        ""
    } else {
        krate.rel_dir.as_str()
    };
    let (readme_rel_path, readme_abs_path) =
        resolve_manifest_relative_path(crawl, readme_base_rel_dir, readme_field.unwrap_or("README.md"));

    CrateBase {
        name,
        cargo_rel_path: krate.cargo_rel_path.clone(),
        cargo_abs_path: krate.cargo_abs_path.clone(),
        cargo: krate.cargo.clone(),
        publish_declared,
        publishable,
        is_binary,
        is_library: is_library_crate(crawl, &krate.rel_dir, &krate.cargo),
        binary_target_names,
        description_present: inherited_string_present(
            package.and_then(|package| package.description.as_ref()),
            workspace_package.and_then(|workspace| workspace.description.as_deref()),
        ),
        license_present: inherited_string_present(
            package.and_then(|package| package.license.as_ref()),
            workspace_package.and_then(|workspace| workspace.license.as_deref()),
        ) || inherited_string_present(
            package.and_then(|package| package.license_file.as_ref()),
            workspace_package.and_then(|workspace| workspace.license_file.as_deref()),
        ),
        repository_present: inherited_string_present(
            package.and_then(|package| package.repository.as_ref()),
            workspace_package.and_then(|workspace| workspace.repository.as_deref()),
        ),
        readme_declared_false,
        readme_rel_path: readme_rel_path.clone(),
        readme_abs_path,
        readme_exists: !readme_declared_false && file_exists(crawl, &readme_rel_path),
        keywords_count: inherited_vec_count(
            package.and_then(|package| package.keywords.as_ref()),
            workspace_package.map(|workspace| workspace.keywords.as_slice()),
        ),
        categories_count: inherited_vec_count(
            package.and_then(|package| package.categories.as_ref()),
            workspace_package.map(|workspace| workspace.categories.as_slice()),
        ),
        version_string: version_string(package, workspace_package),
        workspace_version: matches!(
            package.and_then(|package| package.version.as_ref()),
            Some(InheritableValue::Inherit(_))
        ),
        version_valid: version_string(package, workspace_package)
            .as_deref()
            .is_some_and(|version| Version::parse(version).is_ok()),
        docs_rs_present: package
            .and_then(|package| package.metadata.as_ref())
            .and_then(docs_rs_table)
            .is_some_and(has_supported_docs_rs_settings),
        include_exclude_present: package.is_some_and(|package| {
            package
                .include
                .as_ref()
                .is_some_and(|value| matches!(value, InheritableValue::Value(values) if !values.is_empty()))
                || package
                    .exclude
                    .as_ref()
                    .is_some_and(|value| matches!(value, InheritableValue::Value(values) if !values.is_empty()))
        }),
        has_binstall_metadata: package
            .and_then(|package| package.metadata.as_ref())
            .and_then(|metadata| metadata.get("binstall"))
            .and_then(|value| value.as_table())
            .is_some(),
    }
}

fn parse_release_plz(
    crawl: &G3RsWorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> (bool, String, Option<release_plz_toml_parser::ReleasePlzToml>, BTreeSet<String>) {
    let rel_path = "release-plz.toml".to_owned();
    let Some(entry) = crate::select::select_release_plz_toml(crawl) else {
        return (false, rel_path, None, BTreeSet::new());
    };
    if !entry.readable {
        failures.push(input_failure(
            &entry.path.rel_path,
            "Failed to read release-plz.toml: file is not readable.",
        ));
        return (true, rel_path, None, BTreeSet::new());
    }
    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to read release-plz.toml: {error}"),
            ));
            return (true, rel_path, None, BTreeSet::new());
        }
    };
    match crate::parse::parse_release_plz_toml(&content, &entry.path.abs_path) {
        Ok(parsed) => {
            let package_names = parsed
                .package
                .iter()
                .filter_map(|package| package.name.clone())
                .collect::<BTreeSet<_>>();
            (true, rel_path, Some(parsed), package_names)
        }
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to parse release-plz.toml: {error}"),
            ));
            (true, rel_path, None, BTreeSet::new())
        }
    }
}

fn parse_cliff(
    crawl: &G3RsWorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> (bool, String, Option<cliff_toml_parser::CliffToml>) {
    let rel_path = "cliff.toml".to_owned();
    let Some(entry) = crate::select::select_cliff_toml(crawl) else {
        return (false, rel_path, None);
    };
    if !entry.readable {
        failures.push(input_failure(
            &entry.path.rel_path,
            "Failed to read cliff.toml: file is not readable.",
        ));
        return (true, rel_path, None);
    }
    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to read cliff.toml: {error}"),
            ));
            return (true, rel_path, None);
        }
    };
    match crate::parse::parse_cliff_toml(&content, &entry.path.abs_path) {
        Ok(parsed) => (true, rel_path, Some(parsed)),
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to parse cliff.toml: {error}"),
            ));
            (true, rel_path, None)
        }
    }
}

fn collect_workflows(
    crawl: &G3RsWorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Vec<WorkflowFacts> {
    let mut workflows = Vec::new();
    for entry in crate::select::select_workflow_entries(crawl) {
        if !entry.readable {
            failures.push(input_failure(
                &entry.path.rel_path,
                "Failed to read workflow YAML: file is not readable.",
            ));
            continue;
        }
        let content = match crate::parse::read_to_string(&entry.path.abs_path) {
            Ok(content) => content,
            Err(error) => {
                failures.push(input_failure(
                    &entry.path.rel_path,
                    format!("Failed to read workflow YAML: {error}"),
                ));
                continue;
            }
        };
        let parsed = match crate::parse::parse_workflow_yaml(&content, &entry.path.abs_path) {
            Ok(parsed) => parsed,
            Err(error) => {
                failures.push(input_failure(
                    &entry.path.rel_path,
                    format!("Failed to parse workflow YAML: {error}"),
                ));
                continue;
            }
        };
        workflows.push(WorkflowFacts {
            rel_path: entry.path.rel_path.clone(),
            analysis: extract_workflow_analysis(&parsed),
        });
    }
    workflows
}

#[derive(Debug, Clone)]
struct DependencyEdge {
    dep_name: String,
    dep_package_name: String,
    section_label: String,
    target_label: Option<String>,
    has_path: bool,
    path_target_kind: Option<G3RsReleasePathTargetKind>,
    version_req: Option<String>,
}

fn dependency_edges(
    crawl: &G3RsWorkspaceCrawl,
    source_manifest_abs_path: &Path,
    cargo: &CargoToml,
    workspace_dependencies: &BTreeMap<String, Dependency>,
) -> Vec<DependencyEdge> {
    let mut edges = Vec::new();
    collect_dependency_edges(
        crawl,
        source_manifest_abs_path.parent().unwrap_or(source_manifest_abs_path),
        &cargo.dependencies,
        "dependencies",
        None,
        workspace_dependencies,
        &mut edges,
    );
    collect_dependency_edges(
        crawl,
        source_manifest_abs_path.parent().unwrap_or(source_manifest_abs_path),
        &cargo.build_dependencies,
        "build-dependencies",
        None,
        workspace_dependencies,
        &mut edges,
    );
    for (target_name, target) in &cargo.target {
        collect_target_dependency_edges(
            crawl,
            source_manifest_abs_path.parent().unwrap_or(source_manifest_abs_path),
            target,
            target_name,
            workspace_dependencies,
            &mut edges,
        );
    }
    edges
}

fn collect_target_dependency_edges(
    crawl: &G3RsWorkspaceCrawl,
    source_manifest_dir: &Path,
    target: &TargetDependencyTables,
    target_name: &str,
    workspace_dependencies: &BTreeMap<String, Dependency>,
    edges: &mut Vec<DependencyEdge>,
) {
    collect_dependency_edges(
        crawl,
        source_manifest_dir,
        &target.dependencies,
        "dependencies",
        Some(target_name),
        workspace_dependencies,
        edges,
    );
    collect_dependency_edges(
        crawl,
        source_manifest_dir,
        &target.build_dependencies,
        "build-dependencies",
        Some(target_name),
        workspace_dependencies,
        edges,
    );
}

fn collect_dependency_edges(
    crawl: &G3RsWorkspaceCrawl,
    source_manifest_dir: &Path,
    dependencies: &BTreeMap<String, Dependency>,
    section_label: &str,
    target_label: Option<&str>,
    workspace_dependencies: &BTreeMap<String, Dependency>,
    edges: &mut Vec<DependencyEdge>,
) {
    for (dep_name, dep) in dependencies {
        let workspace_detail = workspace_dependencies.get(dep_name);
        let workspace_inherited = matches!(
            dep,
            Dependency::Detailed(detail) if detail.workspace == Some(true)
        );
        let path_target_kind = dependency_path(dep)
            .as_deref()
            .map(|path| classify_dependency_path(crawl, source_manifest_dir, path))
            .or_else(|| {
                if workspace_inherited {
                    workspace_detail.and_then(dependency_path).as_deref().map(|path| {
                        classify_dependency_path(crawl, &crawl.root_abs_path, path)
                    })
                } else {
                    None
                }
            });
        let has_path = dependency_path(dep).is_some()
            || (workspace_inherited && workspace_detail.and_then(dependency_path).is_some());
        let dep_package_name = dependency_package(dep)
            .or_else(|| {
                if workspace_inherited {
                    workspace_detail.and_then(dependency_package)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| dep_name.clone());
        let version_req = dependency_version(dep).or_else(|| {
            if workspace_inherited {
                workspace_detail.and_then(dependency_version)
            } else {
                None
            }
        });

        edges.push(DependencyEdge {
            dep_name: dep_name.clone(),
            dep_package_name,
            section_label: section_label.to_owned(),
            target_label: target_label.map(str::to_owned),
            has_path,
            path_target_kind,
            version_req,
        });
    }
}

fn dependency_path(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => None,
        Dependency::Detailed(detail) => detail.path.clone(),
    }
}

fn dependency_package(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => None,
        Dependency::Detailed(detail) => detail.package.clone(),
    }
}

fn dependency_version(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(version) => Some(version.clone()),
        Dependency::Detailed(detail) => detail.version.clone(),
    }
}

fn publishable(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    let Some(package) = package else {
        return false;
    };

    match package.publish.as_ref() {
        None => false,
        Some(InheritableValue::Value(VecStringOrBool::Bool(false))) => false,
        Some(InheritableValue::Value(VecStringOrBool::VecString(values))) => !values.is_empty(),
        Some(InheritableValue::Value(VecStringOrBool::Bool(true))) => true,
        Some(InheritableValue::Inherit(_)) => match workspace_package.and_then(|workspace| workspace.publish.as_ref()) {
            None => false,
            Some(VecStringOrBool::Bool(false)) => false,
            Some(VecStringOrBool::VecString(values)) => !values.is_empty(),
            Some(VecStringOrBool::Bool(true)) => true,
        },
    }
}

fn publish_declared(package: Option<&PackageSection>) -> bool {
    package
        .and_then(|package| package.publish.as_ref())
        .is_some()
}

fn inherited_string_present(
    value: Option<&InheritableValue<String>>,
    workspace_value: Option<&str>,
) -> bool {
    match value {
        Some(InheritableValue::Value(value)) => !value.trim().is_empty(),
        Some(InheritableValue::Inherit(_)) => workspace_value.is_some_and(|value| !value.trim().is_empty()),
        None => false,
    }
}

fn inherited_vec_count(
    value: Option<&InheritableValue<Vec<String>>>,
    workspace_values: Option<&[String]>,
) -> Option<usize> {
    match value {
        Some(InheritableValue::Value(values)) => Some(values.len()),
        Some(InheritableValue::Inherit(_)) => workspace_values.map(|values| values.len()),
        None => None,
    }
}

fn version_string(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> Option<String> {
    match package.and_then(|package| package.version.as_ref()) {
        Some(InheritableValue::Value(value)) => Some(value.clone()),
        Some(InheritableValue::Inherit(_)) => workspace_package.and_then(|workspace| workspace.version.clone()),
        None => None,
    }
}

fn readme_declared_false(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    match package.and_then(|package| package.readme.as_ref()) {
        Some(InheritableValue::Value(StringOrBool::Bool(false))) => true,
        Some(InheritableValue::Inherit(_)) => matches!(
            workspace_package.and_then(|workspace| workspace.readme.as_ref()),
            Some(StringOrBool::Bool(false))
        ),
        _ => false,
    }
}

fn readme_path_field<'a>(
    package: Option<&'a PackageSection>,
    workspace_package: Option<&'a WorkspacePackageSection>,
) -> (Option<&'a str>, bool) {
    match package.and_then(|package| package.readme.as_ref()) {
        Some(InheritableValue::Value(StringOrBool::String(path))) => (Some(path.as_str()), false),
        Some(InheritableValue::Inherit(_)) => (
            workspace_package
                .and_then(|workspace| workspace.readme.as_ref())
                .and_then(|value| match value {
                    StringOrBool::String(path) => Some(path.as_str()),
                    StringOrBool::Bool(_) => None,
                }),
            true,
        ),
        _ => (None, false),
    }
}

fn docs_rs_table(metadata: &toml::Value) -> Option<&toml::map::Map<String, toml::Value>> {
    metadata
        .get("docs.rs")
        .and_then(|value| value.as_table())
        .or_else(|| {
            metadata
                .get("docs")
                .and_then(|docs| docs.as_table())
                .and_then(|docs| docs.get("rs"))
                .and_then(|value| value.as_table())
        })
}

fn has_supported_docs_rs_settings(table: &toml::map::Map<String, toml::Value>) -> bool {
    [
        "all-features",
        "features",
        "no-default-features",
        "default-target",
        "targets",
        "rustdoc-args",
        "cargo-args",
    ]
    .iter()
    .any(|key| table.contains_key(*key))
}

fn is_library_crate(crawl: &G3RsWorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
    cargo.lib.is_some() || file_exists(crawl, &join_under_root(rel_dir, "src/lib.rs"))
}

fn is_binary_crate(crawl: &G3RsWorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
    if !cargo.bin.is_empty() {
        return true;
    }
    let autobins_disabled = cargo
        .package
        .as_ref()
        .and_then(|package| package.autobins)
        .is_some_and(|autobins| !autobins);
    !autobins_disabled
        && (file_exists(crawl, &join_under_root(rel_dir, "src/main.rs"))
            || autodiscovered_bin_exists(crawl, rel_dir))
}

fn binary_target_names(
    crawl: &G3RsWorkspaceCrawl,
    rel_dir: &str,
    cargo: &CargoToml,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    for bin in &cargo.bin {
        if let Some(name) = bin.name.clone() {
            let _ = names.insert(name);
            continue;
        }
        if let Some(path) = bin.path.as_deref()
            && let Some(name) = binary_name_from_path(path)
        {
            let _ = names.insert(name);
        }
    }

    let autobins_disabled = cargo
        .package
        .as_ref()
        .and_then(|package| package.autobins)
        .is_some_and(|autobins| !autobins);
    if autobins_disabled {
        return names;
    }

    if file_exists(crawl, &join_under_root(rel_dir, "src/main.rs"))
        && let Some(package_name) = cargo.package.as_ref().and_then(|package| package.name.as_ref())
    {
        let _ = names.insert(package_name.clone());
    }

    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    for file in direct_child_files(crawl, &src_bin_rel) {
        if let Some(name) = binary_name_from_path(&file) {
            let _ = names.insert(name);
        }
    }
    for dir in direct_child_dirs(crawl, &src_bin_rel) {
        if file_exists(crawl, &join_under_root(&src_bin_rel, &format!("{dir}/main.rs"))) {
            let _ = names.insert(dir);
        }
    }

    names
}

fn autodiscovered_bin_exists(crawl: &G3RsWorkspaceCrawl, rel_dir: &str) -> bool {
    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    direct_child_files(crawl, &src_bin_rel)
        .iter()
        .any(|file| file.ends_with(".rs"))
        || direct_child_dirs(crawl, &src_bin_rel)
            .iter()
            .any(|dir| file_exists(crawl, &join_under_root(&src_bin_rel, &format!("{dir}/main.rs"))))
}

fn binary_name_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);

    if path.file_name().and_then(|name| name.to_str()) == Some("main.rs") {
        return path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned);
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .map(str::to_owned)
}

fn resolve_manifest_relative_path(
    crawl: &G3RsWorkspaceCrawl,
    manifest_rel_dir: &str,
    relative: &str,
) -> (String, PathBuf) {
    let joined = if manifest_rel_dir.is_empty() {
        relative.to_owned()
    } else {
        format!("{manifest_rel_dir}/{relative}")
    };
    let rel = normalize_relative_path(Path::new(&joined));
    let abs = crawl
        .entry(&rel)
        .map(|entry| entry.path.abs_path.clone())
        .unwrap_or_else(|| crawl.root_abs_path.join(&rel));
    (rel, abs)
}

fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
}

fn classify_dependency_path(
    crawl: &G3RsWorkspaceCrawl,
    base_dir: &Path,
    relative: &str,
) -> G3RsReleasePathTargetKind {
    let normalized_target = normalize_absolute_path(&base_dir.join(relative));
    let normalized_root = normalize_absolute_path(&crawl.root_abs_path);
    if normalized_target.starts_with(&normalized_root) {
        G3RsReleasePathTargetKind::InWorkspace
    } else {
        G3RsReleasePathTargetKind::OutsideWorkspace
    }
}

fn normalize_absolute_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
        }
    }
    normalized
}

fn join_under_root(root_rel_dir: &str, child: &str) -> String {
    if root_rel_dir.is_empty() {
        child.to_owned()
    } else {
        format!("{root_rel_dir}/{child}")
    }
}

fn file_exists(crawl: &G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    crawl.entry(rel_path)
        .is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
}

fn direct_child_files(crawl: &G3RsWorkspaceCrawl, dir_rel: &str) -> Vec<String> {
    let prefix = if dir_rel.is_empty() {
        String::new()
    } else {
        format!("{dir_rel}/")
    };

    crawl.entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| entry.path.rel_path.strip_prefix(&prefix))
        .filter(|rest| !rest.is_empty() && !rest.contains('/'))
        .map(str::to_owned)
        .collect()
}

fn direct_child_dirs(crawl: &G3RsWorkspaceCrawl, dir_rel: &str) -> Vec<String> {
    let prefix = if dir_rel.is_empty() {
        String::new()
    } else {
        format!("{dir_rel}/")
    };

    let dirs = crawl
        .entries
        .iter()
        .filter_map(|entry| entry.path.rel_path.strip_prefix(&prefix))
        .filter_map(|rest| rest.split_once('/').map(|(first, _)| first))
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    dirs.into_iter().collect()
}

fn release_profile_settings(raw_root: &toml::Value) -> Vec<String> {
    raw_root
        .get("profile")
        .and_then(|value| value.get("release"))
        .and_then(|value| value.as_table())
        .map(|table| {
            table
                .iter()
                .map(|(key, value)| format!("{key} = {value}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn publish_setting_string(raw_root: &toml::Value) -> Option<String> {
    let publish = raw_root
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("publish"))
        .or_else(|| {
            raw_root
                .get("package")
                .and_then(|value| value.get("publish"))
        })?;

    Some(match publish {
        toml::Value::Boolean(value) => value.to_string(),
        toml::Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(|value| format!("\"{value}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => publish.to_string(),
    })
}

fn version_requirement_satisfied(actual: &str, req: &str) -> bool {
    let Ok(actual) = Version::parse(actual) else {
        return false;
    };
    let normalized = if req.trim_start().starts_with(['^', '~', '>', '<', '=']) {
        req.trim().to_owned()
    } else {
        format!("^{req}")
    };
    let Ok(req) = VersionReq::parse(&normalized) else {
        return false;
    };
    req.matches(&actual)
}

fn tool_is_available(tool: &str, path_env: Option<&OsStr>) -> bool {
    let Some(path_env) = path_env else {
        return false;
    };

    std::env::split_paths(path_env).any(|dir| {
        let candidate = dir.join(tool);
        let Ok(metadata) = std::fs::metadata(&candidate) else {
            return false;
        };
        if !metadata.is_file() {
            return false;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            metadata.permissions().mode() & 0o111 != 0
        }
        #[cfg(not(unix))]
        {
            true
        }
    })
}

fn run_publish_dry_run(manifest_path: &Path) -> G3RsReleaseDryRunOutcome {
    let manifest_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let manifest_name = manifest_path
        .file_name()
        .unwrap_or_else(|| OsStr::new("Cargo.toml"));

    match Command::new("cargo")
        .args(["publish", "--dry-run", "--manifest-path"])
        .arg(manifest_name)
        .current_dir(manifest_dir)
        .output()
    {
        Ok(output) if output.status.success() => G3RsReleaseDryRunOutcome::Passed,
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let excerpt = if stderr.trim().is_empty() {
                stdout
                    .lines()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join("; ")
            } else {
                stderr
                    .lines()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            G3RsReleaseDryRunOutcome::Failed(excerpt)
        }
        Err(error) => G3RsReleaseDryRunOutcome::Failed(error.to_string()),
    }
}

fn input_failure(rel_path: impl Into<String>, message: impl Into<String>) -> G3RsReleaseInputFailure {
    G3RsReleaseInputFailure {
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

fn push_all_failures(
    config_failures: &mut Vec<G3RsReleaseInputFailure>,
    filetree_failures: &mut Vec<G3RsReleaseInputFailure>,
    source_failures: &mut Vec<G3RsReleaseInputFailure>,
    rel_path: impl Into<String>,
    message: impl Into<String>,
) {
    let rel_path = rel_path.into();
    let message = message.into();
    config_failures.push(input_failure(&rel_path, &message));
    filetree_failures.push(input_failure(&rel_path, &message));
    source_failures.push(input_failure(rel_path, message));
}
