#![allow(
    clippy::missing_errors_doc,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    reason = "ingestion entry points return crate-defined Error enums whose variants are self-documenting; argument count reflects the apparch fact tuple that flows through ingestion stages, splitting it would obscure the pipeline; line count reflects the dependency-classification fanout per Cargo.toml shape; needless_pass_by_value applies to fixture builders that own the rust_policy so callers can pass owned variants inline"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use cargo_toml_parser::{types::CargoToml, types::Dependency};
use g3rs_apparch_types as apparch;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::error::G3RsApparchIngestionError;
use super::model::{CrateRecord, DependencyCollections};
use super::workspace::{collect_workspace_crates, load_workspace_root};
use crate::view::CrawlView;

#[cfg(test)]
#[path = "config_tests/mod.rs"]
mod config_tests;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<apparch::G3RsApparchConfigChecksInput, G3RsApparchIngestionError> {
    let view = CrawlView::new(crawl);
    let workspace = load_workspace_root(&view)?;
    let records = collect_workspace_crates(&view, &workspace)?;
    let dependencies = collect_dependency_collections(&records, &workspace.cargo);
    let patch_bypasses = collect_patch_bypasses(&view, &records, &workspace.cargo);
    let crates_by_path = records
        .iter()
        .map(|record| (record.krate.cargo_rel_path.clone(), record.krate.clone()))
        .collect::<BTreeMap<_, _>>();
    let crate_dependency_checks =
        build_crate_dependency_checks(&records, &dependencies, &crates_by_path);
    let crate_purity_checks =
        build_crate_purity_checks(&records, &dependencies, workspace.rust_policy.clone());
    let patch_bypass_checks = patch_bypasses
        .into_iter()
        .map(|patch| apparch::G3RsApparchPatchBypassChecksInput {
            patch,
            rust_policy: workspace.rust_policy.clone(),
        })
        .collect();
    let same_layer_cycles_check = build_same_layer_cycles_check(&dependencies, &crates_by_path);

    Ok(apparch::G3RsApparchConfigChecksInput {
        crate_dependency_checks,
        crate_purity_checks,
        patch_bypass_checks,
        same_layer_cycles_check,
    })
}

fn build_crate_dependency_checks(
    records: &[CrateRecord],
    dependencies: &DependencyCollections,
    crates_by_path: &BTreeMap<String, apparch::G3RsApparchCrate>,
) -> Vec<apparch::G3RsApparchCrateDependencyChecksInput> {
    let mut edges_by_source = BTreeMap::<String, Vec<&apparch::G3RsApparchDependencyEdge>>::new();
    for edge in &dependencies.internal_edges {
        edges_by_source
            .entry(edge.from_cargo_rel_path.clone())
            .or_default()
            .push(edge);
    }

    records
        .iter()
        .map(|record| apparch::G3RsApparchCrateDependencyChecksInput {
            krate: record.krate.clone(),
            internal_dependencies: edges_by_source
                .get(&record.krate.cargo_rel_path)
                .into_iter()
                .flat_map(|edges| edges.iter().copied())
                .filter_map(|edge| {
                    crates_by_path
                        .get(&edge.to_cargo_rel_path)
                        .cloned()
                        .map(|target| apparch::G3RsApparchBoundDependency {
                            dep_name: edge.dep_name.clone(),
                            kind: edge.kind,
                            target,
                        })
                })
                .collect(),
        })
        .collect()
}

fn build_crate_purity_checks(
    records: &[CrateRecord],
    dependencies: &DependencyCollections,
    rust_policy: apparch::G3RsApparchRustPolicyState,
) -> Vec<apparch::G3RsApparchCratePurityChecksInput> {
    let mut external_by_source =
        BTreeMap::<String, Vec<apparch::G3RsApparchExternalDependency>>::new();
    for dependency in &dependencies.external_dependencies {
        external_by_source
            .entry(dependency.cargo_rel_path.clone())
            .or_default()
            .push(dependency.clone());
    }

    records
        .iter()
        .map(|record| apparch::G3RsApparchCratePurityChecksInput {
            krate: record.krate.clone(),
            external_dependencies: external_by_source
                .remove(&record.krate.cargo_rel_path)
                .unwrap_or_default(),
            rust_policy: rust_policy.clone(),
        })
        .collect()
}

fn build_same_layer_cycles_check(
    dependencies: &DependencyCollections,
    crates_by_path: &BTreeMap<String, apparch::G3RsApparchCrate>,
) -> apparch::G3RsApparchSameLayerCyclesChecksInput {
    let edges = dependencies
        .internal_edges
        .iter()
        .filter(|edge| !edge.kind.is_dev())
        .filter_map(|edge| {
            let from = crates_by_path.get(&edge.from_cargo_rel_path)?;
            let to = crates_by_path.get(&edge.to_cargo_rel_path)?;
            let (Some(from_layer), Some(to_layer)) = (from.layer, to.layer) else {
                return None;
            };
            if from_layer != to_layer {
                return None;
            }
            Some(apparch::G3RsApparchSameLayerDependencyEdge {
                from: from.clone(),
                to: to.clone(),
            })
        })
        .collect();

    apparch::G3RsApparchSameLayerCyclesChecksInput { edges }
}

fn collect_dependency_collections(
    records: &[CrateRecord],
    root_cargo: &CargoToml,
) -> DependencyCollections {
    let crates_by_name = records
        .iter()
        .map(|record| {
            (
                record.krate.crate_name.clone(),
                record.krate.cargo_rel_path.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let workspace_dependencies = root_cargo
        .workspace
        .as_ref()
        .map(|workspace| &workspace.dependencies);

    let mut internal_edges = BTreeSet::new();
    let mut external_dependencies = BTreeSet::new();
    for record in records {
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.dependencies,
            workspace_dependencies,
            &crates_by_name,
            apparch::G3RsApparchDependencyKind::Dependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.dev_dependencies,
            workspace_dependencies,
            &crates_by_name,
            apparch::G3RsApparchDependencyKind::DevDependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.build_dependencies,
            workspace_dependencies,
            &crates_by_name,
            apparch::G3RsApparchDependencyKind::BuildDependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        for target in record.cargo.target.values() {
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.dependencies,
                workspace_dependencies,
                &crates_by_name,
                apparch::G3RsApparchDependencyKind::TargetDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.dev_dependencies,
                workspace_dependencies,
                &crates_by_name,
                apparch::G3RsApparchDependencyKind::TargetDevDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.build_dependencies,
                workspace_dependencies,
                &crates_by_name,
                apparch::G3RsApparchDependencyKind::TargetBuildDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
        }
    }

    DependencyCollections {
        internal_edges: internal_edges
            .into_iter()
            .map(|(from_cargo_rel_path, to_cargo_rel_path, dep_name, kind)| {
                apparch::G3RsApparchDependencyEdge {
                    from_cargo_rel_path,
                    to_cargo_rel_path,
                    dep_name,
                    kind,
                }
            })
            .collect(),
        external_dependencies: external_dependencies
            .into_iter()
            .map(
                |(cargo_rel_path, dep_name, kind)| apparch::G3RsApparchExternalDependency {
                    cargo_rel_path,
                    dep_name,
                    kind,
                },
            )
            .collect(),
    }
}

fn collect_dependency_table(
    from_cargo_rel_path: &str,
    dependencies: &BTreeMap<String, Dependency>,
    workspace_dependencies: Option<&BTreeMap<String, Dependency>>,
    crates_by_name: &BTreeMap<String, String>,
    kind: apparch::G3RsApparchDependencyKind,
    internal_edges: &mut BTreeSet<(String, String, String, apparch::G3RsApparchDependencyKind)>,
    external_dependencies: &mut BTreeSet<(String, String, apparch::G3RsApparchDependencyKind)>,
) {
    for (dep_name, dependency) in dependencies {
        let package_name = dependency_package(dep_name, dependency, workspace_dependencies);
        if let Some(to_cargo_rel_path) = crates_by_name.get(&package_name) {
            let _ = internal_edges.insert((
                from_cargo_rel_path.to_owned(),
                to_cargo_rel_path.clone(),
                package_name,
                kind,
            ));
        } else {
            let _ =
                external_dependencies.insert((from_cargo_rel_path.to_owned(), package_name, kind));
        }
    }
}

fn dependency_package(
    dep_name: &str,
    dependency: &Dependency,
    workspace_dependencies: Option<&BTreeMap<String, Dependency>>,
) -> String {
    match dependency {
        Dependency::Simple(_) => dep_name.to_owned(),
        Dependency::Detailed(detail) => {
            if detail.workspace == Some(true) {
                if let Some(workspace_dep) =
                    workspace_dependencies.and_then(|deps| deps.get(dep_name))
                {
                    return dependency_package(dep_name, workspace_dep, None);
                }
            }
            detail
                .package
                .clone()
                .unwrap_or_else(|| dep_name.to_owned())
        }
    }
}

fn collect_patch_bypasses(
    view: &CrawlView<'_>,
    records: &[CrateRecord],
    root_cargo: &CargoToml,
) -> Vec<apparch::G3RsApparchPatchBypass> {
    let records_by_cargo_rel_path = records
        .iter()
        .map(|record| (record.krate.cargo_rel_path.clone(), &record.krate))
        .collect::<BTreeMap<_, _>>();
    let mut patch_bypasses = BTreeSet::new();

    for (registry, patch_table) in &root_cargo.patch {
        for (name, dependency) in patch_table {
            let Some(target_cargo_rel_path) =
                resolve_dependency_to_cargo_rel_path(view, dependency)
            else {
                continue;
            };
            let Some(target) = records_by_cargo_rel_path
                .get(&target_cargo_rel_path)
                .copied()
            else {
                continue;
            };
            let _ = patch_bypasses.insert((
                format!("patch.{registry}.{name}"),
                apparch::G3RsApparchPatchKind::Patch,
                target.cargo_rel_path.clone(),
                target.rel_dir.clone(),
                target.layer,
            ));
        }
    }

    for (name, dependency) in &root_cargo.replace {
        let Some(target_cargo_rel_path) = resolve_dependency_to_cargo_rel_path(view, dependency)
        else {
            continue;
        };
        let Some(target) = records_by_cargo_rel_path
            .get(&target_cargo_rel_path)
            .copied()
        else {
            continue;
        };
        let _ = patch_bypasses.insert((
            format!("replace.{name}"),
            apparch::G3RsApparchPatchKind::Replace,
            target.cargo_rel_path.clone(),
            target.rel_dir.clone(),
            target.layer,
        ));
    }

    patch_bypasses
        .into_iter()
        .map(
            |(key, kind, target_cargo_rel_path, target_rel_dir, target_layer)| {
                apparch::G3RsApparchPatchBypass {
                    cargo_rel_path: "Cargo.toml".to_owned(),
                    key,
                    kind,
                    target_cargo_rel_path,
                    target_rel_dir,
                    target_layer,
                }
            },
        )
        .collect()
}

fn resolve_dependency_to_cargo_rel_path(
    view: &CrawlView<'_>,
    dependency: &Dependency,
) -> Option<String> {
    let Dependency::Detailed(detail) = dependency else {
        return None;
    };
    let path = detail.path.as_deref()?;
    let normalized = normalize_relative_path(path);
    let direct = if normalized.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        normalized.clone()
    };
    if view.file_exists(&direct) {
        return Some(direct);
    }

    let cargo_rel_path = CrawlView::join_rel(&normalized, "Cargo.toml");
    view.file_exists(&cargo_rel_path).then_some(cargo_rel_path)
}

fn normalize_relative_path(path: &str) -> String {
    let mut normalized = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {
                return path.replace('\\', "/");
            }
        }
    }
    normalized.join("/")
}
