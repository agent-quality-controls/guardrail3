use std::collections::BTreeSet;
use std::path::{Component, Path};

use cargo_toml_parser::{CargoToml, Dependency, TargetDependencyTables};
use g3_deps_content_checks_types::G3DepsContentChecksInput;
use glob::Pattern;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::RustProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DependencySectionKind {
    Dependencies,
    BuildDependencies,
    DevDependencies,
}

pub(crate) struct DependencyEntry<'a> {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) section_kind: DependencySectionKind,
    pub(crate) table_label: String,
    pub(crate) dep_package_name: String,
}

pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

pub(crate) fn warn(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn error(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn crate_name(cargo_rel_path: &str, cargo: &CargoToml) -> String {
    cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| fallback_name(cargo_rel_path))
}

pub(crate) fn allowlist_present(input: &G3DepsContentChecksInput) -> bool {
    !input.guardrail_rs.allowed_deps.is_empty()
}

pub(crate) fn allowlisted(input: &G3DepsContentChecksInput, dep_package_name: &str) -> bool {
    input
        .guardrail_rs
        .allowed_deps
        .iter()
        .any(|allowed| allowed == dep_package_name)
}

pub(crate) fn workspace_is_library(input: &G3DepsContentChecksInput) -> bool {
    input.guardrail_rs.profile == Some(RustProfile::Library)
}

pub(crate) fn dependency_entries(input: &G3DepsContentChecksInput) -> Vec<DependencyEntry<'_>> {
    let mut entries = Vec::new();
    let crate_name = crate_name(&input.crate_cargo_rel_path, &input.crate_cargo);

    collect_table_entries(
        input,
        &crate_name,
        &input.crate_cargo.dependencies,
        DependencySectionKind::Dependencies,
        "[dependencies]",
        &mut entries,
    );
    collect_table_entries(
        input,
        &crate_name,
        &input.crate_cargo.build_dependencies,
        DependencySectionKind::BuildDependencies,
        "[build-dependencies]",
        &mut entries,
    );
    collect_table_entries(
        input,
        &crate_name,
        &input.crate_cargo.dev_dependencies,
        DependencySectionKind::DevDependencies,
        "[dev-dependencies]",
        &mut entries,
    );

    for (target_key, target_tables) in &input.crate_cargo.target {
        collect_target_entries(
            input,
            &crate_name,
            target_key,
            target_tables,
            &mut entries,
        );
    }

    entries
}

pub(crate) fn unique_direct_dependency_names(
    input: &G3DepsContentChecksInput,
) -> BTreeSet<String> {
    dependency_entries(input)
        .into_iter()
        .map(|entry| entry.dep_package_name)
        .collect()
}

fn collect_target_entries<'a>(
    input: &'a G3DepsContentChecksInput,
    crate_name: &str,
    target_key: &str,
    target_tables: &'a TargetDependencyTables,
    entries: &mut Vec<DependencyEntry<'a>>,
) {
    let dependencies_label = format!("[target.'{target_key}'.dependencies]");
    collect_table_entries(
        input,
        crate_name,
        &target_tables.dependencies,
        DependencySectionKind::Dependencies,
        &dependencies_label,
        entries,
    );

    let build_label = format!("[target.'{target_key}'.build-dependencies]");
    collect_table_entries(
        input,
        crate_name,
        &target_tables.build_dependencies,
        DependencySectionKind::BuildDependencies,
        &build_label,
        entries,
    );

    let dev_label = format!("[target.'{target_key}'.dev-dependencies]");
    collect_table_entries(
        input,
        crate_name,
        &target_tables.dev_dependencies,
        DependencySectionKind::DevDependencies,
        &dev_label,
        entries,
    );
}

fn collect_table_entries<'a>(
    input: &'a G3DepsContentChecksInput,
    crate_name: &str,
    dependencies: &'a std::collections::BTreeMap<String, Dependency>,
    section_kind: DependencySectionKind,
    table_label: &str,
    entries: &mut Vec<DependencyEntry<'a>>,
) {
    for (alias, dependency) in dependencies {
        if let Some(dep_package_name) =
            resolved_dependency_name(input, alias, dependency, &input.crate_cargo_rel_path)
        {
            entries.push(DependencyEntry {
                crate_name: crate_name.to_owned(),
                cargo_rel_path: &input.crate_cargo_rel_path,
                section_kind,
                table_label: table_label.to_owned(),
                dep_package_name,
            });
        }
    }
}

fn resolved_dependency_name(
    input: &G3DepsContentChecksInput,
    alias: &str,
    dependency: &Dependency,
    cargo_rel_path: &str,
) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => Some(alias.to_owned()),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if detail.workspace == Some(true) {
                return resolve_workspace_dependency(input, alias);
            }

            if let Some(path) = &detail.path {
                let resolved = normalize_dependency_path(cargo_rel_path, path);
                if workspace_declares_member(input, &resolved) {
                    return None;
                }
            }

            Some(fallback_name)
        }
    }
}

fn resolve_workspace_dependency(
    input: &G3DepsContentChecksInput,
    alias: &str,
) -> Option<String> {
    let workspace_dependency = input
        .workspace_cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.dependencies.get(alias))?;

    match workspace_dependency {
        Dependency::Simple(_) => Some(alias.to_owned()),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if let Some(path) = &detail.path {
                let workspace_root = workspace_root_rel_dir(&input.workspace_cargo_rel_path);
                let resolved = normalize_rel_path(workspace_root, path);
                if workspace_declares_member(input, &resolved) {
                    return None;
                }
            }

            Some(fallback_name)
        }
    }
}

fn workspace_declares_member(input: &G3DepsContentChecksInput, resolved_rel_path: &str) -> bool {
    let workspace_root = workspace_root_rel_dir(&input.workspace_cargo_rel_path);
    let Some(relative_to_workspace) = strip_prefix_path(resolved_rel_path, workspace_root) else {
        return false;
    };

    input
        .workspace_cargo
        .workspace
        .as_ref()
        .into_iter()
        .flat_map(|workspace| workspace.members.iter())
        .filter_map(|pattern| Pattern::new(pattern).ok())
        .any(|pattern| pattern.matches(relative_to_workspace))
}

fn workspace_root_rel_dir(cargo_rel_path: &str) -> &str {
    cargo_rel_path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

fn strip_prefix_path<'a>(rel_path: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.is_empty() {
        return Some(rel_path);
    }

    if rel_path == prefix {
        return Some("");
    }

    rel_path
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_prefix('/'))
}

fn normalize_dependency_path(cargo_rel_path: &str, dep_path: &str) -> String {
    let base_rel_dir = cargo_rel_path
        .rsplit_once('/')
        .map_or("", |(base_rel_dir, _)| base_rel_dir);
    normalize_rel_path(base_rel_dir, dep_path)
}

fn normalize_rel_path(base_rel_dir: &str, dep_path: &str) -> String {
    let joined = if base_rel_dir.is_empty() {
        Path::new(dep_path).to_path_buf()
    } else {
        Path::new(base_rel_dir).join(dep_path)
    };

    let mut parts = Vec::new();
    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                if parts.last().is_some_and(|last| last != "..") {
                    let _ = parts.pop();
                } else {
                    parts.push("..".to_owned());
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
        }
    }

    parts.join("/")
}

fn fallback_name(cargo_rel_path: &str) -> String {
    cargo_rel_path
        .rsplit_once('/')
        .map_or("root".to_owned(), |(dir, _)| {
            dir.rsplit('/').next().unwrap_or(dir).to_owned()
        })
}
