use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use cargo_toml_parser::{CargoToml, Dependency, TargetDependencyTables};
use g3_deps_content_checks_types::{
    G3DepsDirectDependencyCapInput, G3DepsLocalPathCargoManifest, G3DepsPolicyContentChecksInput,
};
use glob::Pattern;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_domain_config::types::{CrateConfig, GuardrailConfig};

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

pub(crate) fn allowlist_present(input: &G3DepsPolicyContentChecksInput) -> bool {
    crate_policy(input).and_then(CrateConfig::allowed_deps).is_some()
}

pub(crate) fn allowlisted(
    input: &G3DepsPolicyContentChecksInput,
    dep_package_name: &str,
) -> bool {
    crate_policy(input)
        .and_then(CrateConfig::allowed_deps)
        .is_some_and(|allowed| allowed.iter().any(|dep| dep == dep_package_name))
}

pub(crate) fn workspace_is_library(input: &G3DepsPolicyContentChecksInput) -> bool {
    effective_profile_name(&input.guardrail, crate_rel_dir(&input.crate_cargo_rel_path))
        .is_some_and(|profile| profile == "library")
}

pub(crate) fn dependency_entries_from_policy_input(
    input: &G3DepsPolicyContentChecksInput,
) -> Vec<DependencyEntry<'_>> {
    let lookup = local_path_cargo_lookup(
        &input.local_path_cargo_rel_paths,
        &input.local_path_cargo_manifests,
    );
    dependency_entries_with(
        &input.workspace_cargo_rel_path,
        &input.workspace_cargo,
        &input.crate_cargo_rel_path,
        &input.crate_cargo,
        &lookup,
    )
}

pub(crate) fn dependency_entries_from_cap_input(
    input: &G3DepsDirectDependencyCapInput,
) -> Vec<DependencyEntry<'_>> {
    let lookup = local_path_cargo_lookup(
        &input.local_path_cargo_rel_paths,
        &input.local_path_cargo_manifests,
    );
    dependency_entries_with(
        &input.workspace_cargo_rel_path,
        &input.workspace_cargo,
        &input.crate_cargo_rel_path,
        &input.crate_cargo,
        &lookup,
    )
}

pub(crate) fn unique_direct_dependency_names(
    input: &G3DepsDirectDependencyCapInput,
) -> BTreeSet<String> {
    dependency_entries_from_cap_input(input)
        .into_iter()
        .map(|entry| entry.dep_package_name)
        .collect()
}

fn dependency_entries_with<'a>(
    workspace_cargo_rel_path: &'a str,
    workspace_cargo: &'a CargoToml,
    crate_cargo_rel_path: &'a str,
    crate_cargo: &'a CargoToml,
    local_path_lookup: &LocalPathCargoLookup<'a>,
) -> Vec<DependencyEntry<'a>> {
    let mut entries = Vec::new();
    let crate_name = crate_name(crate_cargo_rel_path, crate_cargo);

    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        &crate_name,
        &crate_cargo.dependencies,
        DependencySectionKind::Dependencies,
        "[dependencies]",
        local_path_lookup,
        &mut entries,
    );
    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        &crate_name,
        &crate_cargo.build_dependencies,
        DependencySectionKind::BuildDependencies,
        "[build-dependencies]",
        local_path_lookup,
        &mut entries,
    );
    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        &crate_name,
        &crate_cargo.dev_dependencies,
        DependencySectionKind::DevDependencies,
        "[dev-dependencies]",
        local_path_lookup,
        &mut entries,
    );

    for (target_key, target_tables) in &crate_cargo.target {
        collect_target_entries(
            workspace_cargo_rel_path,
            workspace_cargo,
            crate_cargo_rel_path,
            &crate_name,
            target_key,
            target_tables,
            local_path_lookup,
            &mut entries,
        );
    }

    entries
}

fn collect_target_entries<'a>(
    workspace_cargo_rel_path: &'a str,
    workspace_cargo: &'a CargoToml,
    crate_cargo_rel_path: &'a str,
    crate_name: &str,
    target_key: &str,
    target_tables: &'a TargetDependencyTables,
    local_path_lookup: &LocalPathCargoLookup<'a>,
    entries: &mut Vec<DependencyEntry<'a>>,
) {
    let dependencies_label = format!("[target.'{target_key}'.dependencies]");
    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        crate_name,
        &target_tables.dependencies,
        DependencySectionKind::Dependencies,
        &dependencies_label,
        local_path_lookup,
        entries,
    );

    let build_label = format!("[target.'{target_key}'.build-dependencies]");
    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        crate_name,
        &target_tables.build_dependencies,
        DependencySectionKind::BuildDependencies,
        &build_label,
        local_path_lookup,
        entries,
    );

    let dev_label = format!("[target.'{target_key}'.dev-dependencies]");
    collect_table_entries(
        workspace_cargo_rel_path,
        workspace_cargo,
        crate_cargo_rel_path,
        crate_name,
        &target_tables.dev_dependencies,
        DependencySectionKind::DevDependencies,
        &dev_label,
        local_path_lookup,
        entries,
    );
}

fn collect_table_entries<'a>(
    workspace_cargo_rel_path: &'a str,
    workspace_cargo: &'a CargoToml,
    crate_cargo_rel_path: &'a str,
    crate_name: &str,
    dependencies: &'a std::collections::BTreeMap<String, Dependency>,
    section_kind: DependencySectionKind,
    table_label: &str,
    local_path_lookup: &LocalPathCargoLookup<'a>,
    entries: &mut Vec<DependencyEntry<'a>>,
) {
    for (alias, dependency) in dependencies {
        if let Some(dep_package_name) = resolved_dependency_name(
            workspace_cargo_rel_path,
            workspace_cargo,
            crate_cargo_rel_path,
            alias,
            dependency,
            local_path_lookup,
        ) {
            entries.push(DependencyEntry {
                crate_name: crate_name.to_owned(),
                cargo_rel_path: crate_cargo_rel_path,
                section_kind,
                table_label: table_label.to_owned(),
                dep_package_name,
            });
        }
    }
}

fn resolved_dependency_name(
    workspace_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    crate_cargo_rel_path: &str,
    alias: &str,
    dependency: &Dependency,
    local_path_lookup: &LocalPathCargoLookup<'_>,
) -> Option<String> {
    match dependency {
        Dependency::Simple(_) => Some(alias.to_owned()),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if detail.workspace == Some(true) {
                return resolve_workspace_dependency(
                    workspace_cargo_rel_path,
                    workspace_cargo,
                    alias,
                    local_path_lookup,
                );
            }

            if let Some(path) = &detail.path {
                let resolved = normalize_dependency_path(crate_cargo_rel_path, path);
                let cargo_rel_path = path_cargo_rel_path(&resolved);
                if local_path_lookup
                    .known_paths
                    .contains(cargo_rel_path.as_str())
                {
                    if workspace_declares_member(
                        workspace_cargo_rel_path,
                        workspace_cargo,
                        &resolved,
                    ) {
                        return None;
                    }
                    if path_is_under_workspace_root(workspace_cargo_rel_path, &resolved) {
                        return None;
                    }
                    return local_path_lookup
                        .manifests
                        .get(cargo_rel_path.as_str())
                        .map(|cargo| crate_name(cargo_rel_path.as_str(), cargo));
                }
                if workspace_declares_member(
                    workspace_cargo_rel_path,
                    workspace_cargo,
                    &resolved,
                ) {
                    return None;
                }
            }

            Some(fallback_name)
        }
    }
}

fn resolve_workspace_dependency(
    workspace_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    alias: &str,
    local_path_lookup: &LocalPathCargoLookup<'_>,
) -> Option<String> {
    let workspace_dependency = workspace_cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.dependencies.get(alias))?;

    match workspace_dependency {
        Dependency::Simple(_) => Some(alias.to_owned()),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if let Some(path) = &detail.path {
                let workspace_root = workspace_root_rel_dir(workspace_cargo_rel_path);
                let resolved = normalize_rel_path(workspace_root, path);
                let cargo_rel_path = path_cargo_rel_path(&resolved);
                if local_path_lookup
                    .known_paths
                    .contains(cargo_rel_path.as_str())
                {
                    if workspace_declares_member(
                        workspace_cargo_rel_path,
                        workspace_cargo,
                        &resolved,
                    ) {
                        return None;
                    }
                    if path_is_under_workspace_root(workspace_cargo_rel_path, &resolved) {
                        return None;
                    }
                    return local_path_lookup
                        .manifests
                        .get(cargo_rel_path.as_str())
                        .map(|cargo| crate_name(cargo_rel_path.as_str(), cargo));
                }
                if workspace_declares_member(
                    workspace_cargo_rel_path,
                    workspace_cargo,
                    &resolved,
                ) {
                    return None;
                }
            }

            Some(fallback_name)
        }
    }
}

fn workspace_declares_member(
    workspace_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    resolved_rel_path: &str,
) -> bool {
    let workspace_root = workspace_root_rel_dir(workspace_cargo_rel_path);
    let Some(relative_to_workspace) = strip_prefix_path(resolved_rel_path, workspace_root) else {
        return false;
    };

    workspace_cargo
        .workspace
        .as_ref()
        .into_iter()
        .flat_map(|workspace| workspace.members.iter())
        .filter_map(|pattern| Pattern::new(pattern).ok())
        .any(|pattern| pattern.matches(relative_to_workspace))
}

struct LocalPathCargoLookup<'a> {
    known_paths: BTreeSet<&'a str>,
    manifests: BTreeMap<&'a str, &'a CargoToml>,
}

fn local_path_cargo_lookup<'a>(
    known_paths: &'a [String],
    manifests: &'a [G3DepsLocalPathCargoManifest],
) -> LocalPathCargoLookup<'a> {
    LocalPathCargoLookup {
        known_paths: known_paths.iter().map(String::as_str).collect(),
        manifests: manifests
            .iter()
            .map(|manifest| (manifest.cargo_rel_path.as_str(), &manifest.cargo))
            .collect(),
    }
}

fn workspace_root_rel_dir(cargo_rel_path: &str) -> &str {
    cargo_rel_path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

fn path_cargo_rel_path(resolved_rel_path: &str) -> String {
    if resolved_rel_path.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{resolved_rel_path}/Cargo.toml")
    }
}

fn path_is_under_workspace_root(
    workspace_cargo_rel_path: &str,
    resolved_rel_path: &str,
) -> bool {
    let workspace_root = workspace_root_rel_dir(workspace_cargo_rel_path);
    if workspace_root.is_empty() {
        return !resolved_rel_path.split('/').any(|segment| segment == "..");
    }

    resolved_rel_path == workspace_root
        || resolved_rel_path
            .strip_prefix(workspace_root)
            .is_some_and(|rest| rest.starts_with('/'))
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

fn crate_policy(input: &G3DepsPolicyContentChecksInput) -> Option<&CrateConfig> {
    crate_policy_from_guardrail(&input.guardrail, crate_rel_dir(&input.crate_cargo_rel_path))
}

fn effective_profile_name<'a>(
    guardrail: &'a GuardrailConfig,
    crate_rel_dir: &'a str,
) -> Option<&'a str> {
    crate_policy_from_guardrail(guardrail, crate_rel_dir)
        .and_then(|cfg| cfg.profile().or_else(|| cfg.type_()))
        .or_else(|| guardrail.profile().map(|profile| profile.name()))
}

fn crate_policy_from_guardrail<'a>(
    guardrail: &'a GuardrailConfig,
    crate_rel_dir: &'a str,
) -> Option<&'a CrateConfig> {
    let rust = guardrail.rust()?;
    match governed_zone_scope(crate_rel_dir) {
        Some(GovernedZoneScope::App(app_name)) => rust.apps().and_then(|apps| apps.get(app_name)),
        Some(GovernedZoneScope::Packages) => rust.packages(),
        None => None,
    }
}

fn crate_rel_dir(cargo_rel_path: &str) -> &str {
    cargo_rel_path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

enum GovernedZoneScope<'a> {
    App(&'a str),
    Packages,
}

fn governed_zone_scope(rel_dir: &str) -> Option<GovernedZoneScope<'_>> {
    let segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() < 2 {
        return None;
    }

    let mut app_names = Vec::new();
    let mut package_hits = 0usize;
    for window in segments.windows(2) {
        match window {
            ["apps", app_name] => app_names.push(*app_name),
            ["packages", _] => package_hits += 1,
            _ => {}
        }
    }

    match (app_names.len(), package_hits) {
        (1, 0) => Some(GovernedZoneScope::App(app_names[0])),
        (0, 1) => Some(GovernedZoneScope::Packages),
        _ => None,
    }
}
