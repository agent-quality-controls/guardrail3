/// Assemble the checks input from selected and parsed data.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

/// Result of resolving a single dependency to its package name. `None` means the
/// dependency is an in-workspace member (excluded from the policy view).
pub(crate) type ResolvedDependencyName = Result<Option<String>, String>;

/// Result of `classify_local_path_dependency`. The outer `Option` distinguishes the
/// "fall through to alias-based resolution" case from a definite answer; the inner
/// `Option<String>` is the resolved package name (or `None` when the dependency is
/// an in-workspace member).
pub(crate) type ResolvedLocalPathDependency = Result<Option<Option<String>>, String>;

use cargo_toml_parser::{types::CargoToml, types::Dependency, types::TargetDependencyTables};
use g3rs_deps_types::{
    G3RsDepsConfigChecksInput, G3RsDepsConfigInputScope, G3RsDepsDependencySection,
    G3RsDepsResolvedDependency,
};
use g3rs_toml_parser::types::Guardrail3RsToml;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum `NormalizedDependencyPath` used by this module.
enum NormalizedDependencyPath {
    /// Variant `Relative`.
    Relative(String),
    /// Variant `Absolute`.
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
    let ctx = WorkspaceCtx {
        crate_cargo_rel_path: crate_cargo_rel_path.as_str(),
        workspace_cargo,
        workspace_member_dirs,
        workspace_root_abs_dir,
    };

    collect_table_entries(
        &ctx,
        &crate_cargo.dependencies,
        G3RsDepsDependencySection::Dependencies,
        "[dependencies]",
        &mut dependencies,
    )?;
    collect_table_entries(
        &ctx,
        &crate_cargo.build_dependencies,
        G3RsDepsDependencySection::BuildDependencies,
        "[build-dependencies]",
        &mut dependencies,
    )?;
    collect_table_entries(
        &ctx,
        &crate_cargo.dev_dependencies,
        G3RsDepsDependencySection::DevDependencies,
        "[dev-dependencies]",
        &mut dependencies,
    )?;

    for (target_key, target_tables) in &crate_cargo.target {
        collect_target_entries(&ctx, target_key, target_tables, &mut dependencies)?;
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

/// Implements `collect target entries`.
fn collect_target_entries(
    ctx: &WorkspaceCtx<'_>,
    target_key: &str,
    target_tables: &TargetDependencyTables,
    entries: &mut Vec<G3RsDepsResolvedDependency>,
) -> Result<(), String> {
    let dependencies_label = format!("[target.'{target_key}'.dependencies]");
    collect_table_entries(
        ctx,
        &target_tables.dependencies,
        G3RsDepsDependencySection::Dependencies,
        &dependencies_label,
        entries,
    )?;

    let build_label = format!("[target.'{target_key}'.build-dependencies]");
    collect_table_entries(
        ctx,
        &target_tables.build_dependencies,
        G3RsDepsDependencySection::BuildDependencies,
        &build_label,
        entries,
    )?;

    let dev_label = format!("[target.'{target_key}'.dev-dependencies]");
    collect_table_entries(
        ctx,
        &target_tables.dev_dependencies,
        G3RsDepsDependencySection::DevDependencies,
        &dev_label,
        entries,
    )?;

    Ok(())
}

/// Bundle of workspace-wide context threaded through dependency resolution helpers.
pub(crate) struct WorkspaceCtx<'ws> {
    /// Path of the calling crate's `Cargo.toml`, relative to the workspace root.
    pub(crate) crate_cargo_rel_path: &'ws str,
    /// Parsed workspace-root `Cargo.toml`.
    pub(crate) workspace_cargo: &'ws CargoToml,
    /// Set of workspace member directories, relative to the workspace root.
    pub(crate) workspace_member_dirs: &'ws BTreeSet<String>,
    /// Absolute directory containing the workspace-root `Cargo.toml`.
    pub(crate) workspace_root_abs_dir: &'ws Path,
}

/// Implements `collect table entries`.
fn collect_table_entries(
    ctx: &WorkspaceCtx<'_>,
    dependencies: &BTreeMap<String, Dependency>,
    section: G3RsDepsDependencySection,
    table_label: &str,
    entries: &mut Vec<G3RsDepsResolvedDependency>,
) -> Result<(), String> {
    for (alias, dependency) in dependencies {
        if let Some(package_name) = resolve_dependency_name(ctx, alias, dependency)? {
            entries.push(G3RsDepsResolvedDependency {
                package_name,
                section,
                table_label: table_label.to_owned(),
            });
        }
    }

    Ok(())
}

/// Implements `resolve dependency name`.
fn resolve_dependency_name(
    ctx: &WorkspaceCtx<'_>,
    alias: &str,
    dependency: &Dependency,
) -> ResolvedDependencyName {
    match dependency {
        Dependency::Simple(_) => Ok(Some(alias.to_owned())),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if detail.workspace == Some(true) {
                return resolve_workspace_dependency(ctx, alias);
            }

            if let Some(path) = &detail.path {
                let resolved = normalize_dependency_path(
                    ctx.crate_cargo_rel_path,
                    path,
                    ctx.workspace_root_abs_dir,
                );
                if let Some(name) = classify_local_path_dependency(
                    resolved,
                    ctx.workspace_member_dirs,
                    alias,
                    "local path dependency",
                    &fallback_name,
                )? {
                    return Ok(name);
                }
            }

            Ok(Some(fallback_name))
        }
    }
}

/// Classifies a normalized dependency path into the resolved package name to record:
/// - `Some(Some(name))` when the dependency is external to the workspace.
/// - `Some(None)` when the dependency resolves to a known workspace member.
/// - `None` when the path falls back to alias-from-detail handling.
///
/// Returns `Err` when the path resolves inside the workspace but does not match
/// any known member.
fn classify_local_path_dependency(
    resolved: NormalizedDependencyPath,
    workspace_member_dirs: &BTreeSet<String>,
    alias: &str,
    kind_label: &str,
    fallback_name: &str,
) -> ResolvedLocalPathDependency {
    match resolved {
        NormalizedDependencyPath::Absolute(_) => Ok(Some(Some(fallback_name.to_owned()))),
        NormalizedDependencyPath::Relative(resolved_rel_path) => {
            if !resolved_path_is_inside_workspace(&resolved_rel_path) {
                return Ok(None);
            }
            if workspace_member_dirs.contains(resolved_rel_path.as_str()) {
                return Ok(Some(None));
            }
            Err(format!(
                "{kind_label} `{alias}` resolves to in-workspace non-member `{resolved_rel_path}`"
            ))
        }
    }
}

/// Implements `resolve workspace dependency`.
fn resolve_workspace_dependency(ctx: &WorkspaceCtx<'_>, alias: &str) -> ResolvedDependencyName {
    let workspace_dependency = ctx
        .workspace_cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.dependencies.get(alias))
        .ok_or_else(|| format!("workspace dependency `{alias}` was requested but not defined"))?;

    match workspace_dependency {
        Dependency::Simple(_) => Ok(Some(alias.to_owned())),
        Dependency::Detailed(detail) => {
            let fallback_name = detail.package.clone().unwrap_or_else(|| alias.to_owned());

            if let Some(path) = &detail.path {
                let resolved = normalize_rel_path("", path, ctx.workspace_root_abs_dir);
                if let Some(name) = classify_local_path_dependency(
                    resolved,
                    ctx.workspace_member_dirs,
                    alias,
                    "workspace dependency",
                    &fallback_name,
                )? {
                    return Ok(name);
                }
            }

            Ok(Some(fallback_name))
        }
    }
}

/// Implements `normalize dependency path`.
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

/// Implements `normalize rel path`.
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
        // Canonicalize both sides so symlinked roots (e.g. macOS `/tmp` -> `/private/tmp`)
        // do not defeat the in-workspace check.
        let canonical_joined = joined.canonicalize().unwrap_or_else(|_| joined.clone());
        let canonical_workspace_root = workspace_root_abs_dir
            .canonicalize()
            .unwrap_or_else(|_| workspace_root_abs_dir.to_path_buf());
        if let Ok(stripped) = canonical_joined.strip_prefix(&canonical_workspace_root) {
            return NormalizedDependencyPath::Relative(normalize_relative_path(stripped));
        }
        return NormalizedDependencyPath::Absolute(canonical_joined.to_string_lossy().into_owned());
    }

    NormalizedDependencyPath::Relative(normalize_relative_path(&joined))
}

/// Implements `normalize relative path`.
fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                if parts.last().is_some_and(|last| last != "..") {
                    let _ = parts.pop();
                } else {
                    parts.push("..".to_owned());
                }
            }
        }
    }

    parts.join("/")
}

/// Implements `resolved path is inside workspace`.
fn resolved_path_is_inside_workspace(resolved_rel_path: &str) -> bool {
    !resolved_rel_path.split('/').any(|segment| segment == "..")
}

/// Implements `crate name`.
fn crate_name(cargo_rel_path: &str, cargo: &CargoToml) -> String {
    cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| fallback_name(cargo_rel_path))
}

/// Implements `fallback name`.
fn fallback_name(cargo_rel_path: &str) -> String {
    let Some((dir, _)) = cargo_rel_path.rsplit_once('/') else {
        return "root".to_owned();
    };
    dir.rsplit('/').next().unwrap_or(dir).to_owned()
}
