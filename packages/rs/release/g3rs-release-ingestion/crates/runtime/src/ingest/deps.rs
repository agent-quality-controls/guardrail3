use std::collections::BTreeMap;
use std::path::Path;

use cargo_toml_parser::types::{CargoToml, Dependency, TargetDependencyTables};
use g3rs_release_types::G3RsReleasePathTargetKind;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

#[derive(Debug, Clone)]
/// `DependencyEdge` value.
pub(super) struct DependencyEdge {
    /// `dep_name` field.
    pub(super) dep_name: String,
    /// `dep_package_name` field.
    pub(super) dep_package_name: String,
    /// `section_label` field.
    pub(super) section_label: String,
    /// `target_label` field.
    pub(super) target_label: Option<String>,
    /// `has_path` field.
    pub(super) has_path: bool,
    /// `path_target_kind` field.
    pub(super) path_target_kind: Option<G3RsReleasePathTargetKind>,
    /// `version_req` field.
    pub(super) version_req: Option<String>,
}

/// `dependency_edges` function.
pub(super) fn dependency_edges(
    crawl: &G3RsWorkspaceCrawl,
    source_manifest_abs_path: &Path,
    cargo: &CargoToml,
    workspace_dependencies: &BTreeMap<String, Dependency>,
) -> Vec<DependencyEdge> {
    let mut edges = Vec::new();
    collect_dependency_edges(
        crawl,
        source_manifest_abs_path
            .parent()
            .unwrap_or(source_manifest_abs_path),
        &cargo.dependencies,
        "dependencies",
        None,
        workspace_dependencies,
        &mut edges,
    );
    collect_dependency_edges(
        crawl,
        source_manifest_abs_path
            .parent()
            .unwrap_or(source_manifest_abs_path),
        &cargo.build_dependencies,
        "build-dependencies",
        None,
        workspace_dependencies,
        &mut edges,
    );
    for (target_name, target) in &cargo.target {
        collect_target_dependency_edges(
            crawl,
            source_manifest_abs_path
                .parent()
                .unwrap_or(source_manifest_abs_path),
            target,
            target_name,
            workspace_dependencies,
            &mut edges,
        );
    }
    edges
}

/// `collect_target_dependency_edges` function.
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

/// `collect_dependency_edges` function.
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
            .map(|path| super::paths::classify_dependency_path(crawl, source_manifest_dir, path))
            .or_else(|| inherited_path_target_kind(crawl, workspace_inherited, workspace_detail));
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

/// Classify the workspace-inherited dependency path, if any.
fn inherited_path_target_kind(
    crawl: &G3RsWorkspaceCrawl,
    workspace_inherited: bool,
    workspace_detail: Option<&Dependency>,
) -> Option<G3RsReleasePathTargetKind> {
    if !workspace_inherited {
        return None;
    }
    workspace_detail
        .and_then(dependency_path)
        .as_deref()
        .map(|path| super::paths::classify_dependency_path(crawl, &crawl.root_abs_path, path))
}

/// `dependency_path` function.
fn dependency_path(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => None,
        Dependency::Detailed(detail) => detail.path.clone(),
    }
}

/// `dependency_package` function.
fn dependency_package(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => None,
        Dependency::Detailed(detail) => detail.package.clone(),
    }
}

/// `dependency_version` function.
fn dependency_version(dependency: &Dependency) -> Option<String> {
    match dependency {
        Dependency::Simple(version) => Some(version.clone()),
        Dependency::Detailed(detail) => detail.version.clone(),
    }
}
