/// Assemble the checks input from selected and parsed data.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use cargo_toml_parser::{types::CargoToml, types::Dependency, types::TargetDependencyTables};
use g3rs_deps_types::{
    G3RsDepsConfigChecksInput, G3RsDepsConfigInputScope, G3RsDepsDependencySection,
    G3RsDepsResolvedDependency,
};
use guardrail3_rs_toml_parser::Guardrail3RsToml;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalizedDependencyPath {
    Relative(String),
    Absolute(String),
}

/// Build one deps config checks input from parsed workspace and member data.
pub(crate) fn assemble(
    crate_cargo_rel_path: String,
    crate_cargo: &CargoToml,
    workspace_cargo: &CargoToml,
    guardrail: &Guardrail3RsToml,
    allowlist_present: bool,
    workspace_member_dirs: &BTreeSet<String>,
    workspace_root_abs_dir: &Path,
) -> Result<G3RsDepsConfigChecksInput, String> {
    let mut dependencies = Vec::new();

    collect_table_entries(
        crate_cargo_rel_path.as_str(),
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &crate_cargo.dependencies,
        G3RsDepsDependencySection::Dependencies,
        "[dependencies]",
        &mut dependencies,
    )?;
    collect_table_entries(
        crate_cargo_rel_path.as_str(),
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &crate_cargo.build_dependencies,
        G3RsDepsDependencySection::BuildDependencies,
        "[build-dependencies]",
        &mut dependencies,
    )?;
    collect_table_entries(
        crate_cargo_rel_path.as_str(),
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &crate_cargo.dev_dependencies,
        G3RsDepsDependencySection::DevDependencies,
        "[dev-dependencies]",
        &mut dependencies,
    )?;

    for (target_key, target_tables) in &crate_cargo.target {
        collect_target_entries(
            crate_cargo_rel_path.as_str(),
            workspace_cargo,
            workspace_member_dirs,
            workspace_root_abs_dir,
            target_key,
            target_tables,
            &mut dependencies,
        )?;
    }

    Ok(G3RsDepsConfigChecksInput {
        scope: G3RsDepsConfigInputScope::CratePolicy,
        crate_name: crate_name(crate_cargo_rel_path.as_str(), crate_cargo),
        crate_cargo_rel_path,
        profile: guardrail.profile,
        allowlist_present,
        allowed_deps: guardrail.allowed_deps.clone(),
        dependencies,
        installed_tools: Vec::new(),
    })
}

fn collect_target_entries(
    crate_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    workspace_member_dirs: &BTreeSet<String>,
    workspace_root_abs_dir: &Path,
    target_key: &str,
    target_tables: &TargetDependencyTables,
    entries: &mut Vec<G3RsDepsResolvedDependency>,
) -> Result<(), String> {
    let dependencies_label = format!("[target.'{target_key}'.dependencies]");
    collect_table_entries(
        crate_cargo_rel_path,
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &target_tables.dependencies,
        G3RsDepsDependencySection::Dependencies,
        &dependencies_label,
        entries,
    )?;

    let build_label = format!("[target.'{target_key}'.build-dependencies]");
    collect_table_entries(
        crate_cargo_rel_path,
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &target_tables.build_dependencies,
        G3RsDepsDependencySection::BuildDependencies,
        &build_label,
        entries,
    )?;

    let dev_label = format!("[target.'{target_key}'.dev-dependencies]");
    collect_table_entries(
        crate_cargo_rel_path,
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
        &target_tables.dev_dependencies,
        G3RsDepsDependencySection::DevDependencies,
        &dev_label,
        entries,
    )?;

    Ok(())
}

fn collect_table_entries(
    crate_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    workspace_member_dirs: &BTreeSet<String>,
    workspace_root_abs_dir: &Path,
    dependencies: &BTreeMap<String, Dependency>,
    section: G3RsDepsDependencySection,
    table_label: &str,
    entries: &mut Vec<G3RsDepsResolvedDependency>,
) -> Result<(), String> {
    for (alias, dependency) in dependencies {
        if let Some(package_name) = resolve_dependency_name(
            crate_cargo_rel_path,
            workspace_cargo,
            workspace_member_dirs,
            workspace_root_abs_dir,
            alias,
            dependency,
        )? {
            entries.push(G3RsDepsResolvedDependency {
                package_name,
                section,
                table_label: table_label.to_owned(),
            });
        }
    }

    Ok(())
}

fn resolve_dependency_name(
    crate_cargo_rel_path: &str,
    workspace_cargo: &CargoToml,
    workspace_member_dirs: &BTreeSet<String>,
    workspace_root_abs_dir: &Path,
    alias: &str,
    dependency: &Dependency,
) -> Result<Option<String>, String> {
    match dependency {
        Dependency::Simple(_) => Ok(Some(alias.to_owned())),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if detail.workspace == Some(true) {
                return resolve_workspace_dependency(
                    workspace_cargo,
                    workspace_member_dirs,
                    workspace_root_abs_dir,
                    alias,
                );
            }

            if let Some(path) = &detail.path {
                let resolved =
                    normalize_dependency_path(crate_cargo_rel_path, path, workspace_root_abs_dir);
                match resolved {
                    NormalizedDependencyPath::Absolute(_) => return Ok(Some(fallback_name)),
                    NormalizedDependencyPath::Relative(resolved_rel_path) => {
                        if resolved_path_is_inside_workspace(&resolved_rel_path) {
                            if workspace_member_dirs.contains(resolved_rel_path.as_str()) {
                                return Ok(None);
                            }
                            return Err(format!(
                                "local path dependency `{alias}` resolves to in-workspace non-member `{resolved_rel_path}`"
                            ));
                        }
                    }
                }
            }

            Ok(Some(fallback_name))
        }
    }
}

fn resolve_workspace_dependency(
    workspace_cargo: &CargoToml,
    workspace_member_dirs: &BTreeSet<String>,
    workspace_root_abs_dir: &Path,
    alias: &str,
) -> Result<Option<String>, String> {
    let workspace_dependency = workspace_cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.dependencies.get(alias))
        .ok_or_else(|| format!("workspace dependency `{alias}` was requested but not defined"))?;

    match workspace_dependency {
        Dependency::Simple(_) => Ok(Some(alias.to_owned())),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if let Some(path) = &detail.path {
                let resolved = normalize_rel_path("", path, workspace_root_abs_dir);
                match resolved {
                    NormalizedDependencyPath::Absolute(_) => return Ok(Some(fallback_name)),
                    NormalizedDependencyPath::Relative(resolved_rel_path) => {
                        if resolved_path_is_inside_workspace(&resolved_rel_path) {
                            if workspace_member_dirs.contains(resolved_rel_path.as_str()) {
                                return Ok(None);
                            }
                            return Err(format!(
                                "workspace dependency `{alias}` resolves to in-workspace non-member `{resolved_rel_path}`"
                            ));
                        }
                    }
                }
            }

            Ok(Some(fallback_name))
        }
    }
}

fn normalize_dependency_path(
    cargo_rel_path: &str,
    dep_path: &str,
    workspace_root_abs_dir: &Path,
) -> NormalizedDependencyPath {
    let base_rel_dir = cargo_rel_path
        .rsplit_once('/')
        .map_or("", |(base_rel_dir, _)| base_rel_dir);
    normalize_rel_path(base_rel_dir, dep_path, workspace_root_abs_dir)
}

fn normalize_rel_path(
    base_rel_dir: &str,
    dep_path: &str,
    workspace_root_abs_dir: &Path,
) -> NormalizedDependencyPath {
    let joined = if base_rel_dir.is_empty() {
        Path::new(dep_path).to_path_buf()
    } else {
        Path::new(base_rel_dir).join(dep_path)
    };

    if joined.is_absolute() {
        if let Ok(stripped) = joined.strip_prefix(workspace_root_abs_dir) {
            return NormalizedDependencyPath::Relative(normalize_relative_path(stripped));
        }
        return NormalizedDependencyPath::Absolute(joined.to_string_lossy().into_owned());
    }

    NormalizedDependencyPath::Relative(normalize_relative_path(&joined))
}

fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
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

fn resolved_path_is_inside_workspace(resolved_rel_path: &str) -> bool {
    !resolved_rel_path.split('/').any(|segment| segment == "..")
}

fn crate_name(cargo_rel_path: &str, cargo: &CargoToml) -> String {
    cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| fallback_name(cargo_rel_path))
}

fn fallback_name(cargo_rel_path: &str) -> String {
    cargo_rel_path
        .rsplit_once('/')
        .map_or("root".to_owned(), |(dir, _)| {
            dir.rsplit('/').next().unwrap_or(dir).to_owned()
        })
}
