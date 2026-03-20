//! Phase 2: Build project structure from crawl results.
//!
//! Takes a flat `CrawlResult` (list of files) and produces a `ProjectMap`
//! (tree of scopes with config file locations and coverage analysis).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Project map types
// ---------------------------------------------------------------------------

/// Complete project structure derived from crawl.
#[derive(Debug)]
pub struct ProjectMap {
    pub root: PathBuf,
    pub rust_scopes: Vec<RustScope>,
    pub ts_scopes: Vec<TsScope>,
    pub root_configs: RootConfigs,
    /// Walk-up config files that shadow a scope's config (enforcement gaps).
    pub shadows: Vec<ShadowWarning>,
}

/// A Rust compilation scope — workspace or standalone crate.
#[derive(Debug)]
pub struct RustScope {
    pub root: PathBuf,
    pub kind: RustScopeKind,
    pub members: Vec<RustMember>,
    pub configs: RustScopeConfigs,
}

#[derive(Debug)]
pub enum RustScopeKind {
    Workspace,
    StandaloneCrate,
}

#[derive(Debug)]
pub struct RustMember {
    pub name: String,
    pub dir: PathBuf,
}

/// Which guardrail config files exist at this scope's root.
#[derive(Debug, Default)]
pub struct RustScopeConfigs {
    pub clippy_toml: Option<PathBuf>,
    pub deny_toml: Option<PathBuf>,
    pub rustfmt_toml: Option<PathBuf>,
    pub cargo_lock: Option<PathBuf>,
    pub rust_toolchains: Vec<PathBuf>,
    pub jscpd_config: Option<PathBuf>,
}

/// A TypeScript scope — app, package, or tool.
#[derive(Debug)]
pub struct TsScope {
    pub path: PathBuf,
    pub kind: TsScopeKind,
    pub name: String,
    pub configs: TsScopeConfigs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TsScopeKind {
    App,
    Package,
    Tool,
}

/// Which config files exist at this TS scope's path.
#[derive(Debug, Default)]
pub struct TsScopeConfigs {
    pub package_json: Option<PathBuf>,
    pub tsconfig: Option<PathBuf>,
    pub eslint_config: Option<PathBuf>,
    pub stylelint_config: Option<PathBuf>,
    pub velite_config: Option<PathBuf>,
    pub next_config: Option<PathBuf>,
}

/// Root-level configs (shared across all scopes).
#[derive(Debug, Default)]
pub struct RootConfigs {
    pub guardrail3_tomls: Vec<PathBuf>,
    pub package_json: Option<PathBuf>,
    pub pnpm_workspaces: Vec<PathBuf>,
    pub eslint_config: Option<PathBuf>,
    pub stylelint_config: Option<PathBuf>,
    pub tsconfig_base: Option<PathBuf>,
    pub npmrc: Option<PathBuf>,
    pub jscpd_config: Option<PathBuf>,
    pub cspell_config: Option<PathBuf>,
    pub prettier_config: Option<PathBuf>,
    pub rust_toolchains: Vec<PathBuf>,
    pub release_plz_tomls: Vec<PathBuf>,
    pub cliff_tomls: Vec<PathBuf>,
    pub pre_commit_hooks: Vec<PathBuf>,
    pub license_files: Vec<PathBuf>,
    pub claude_mds: Vec<PathBuf>,
    pub cargo_mutants_tomls: Vec<PathBuf>,
    pub github_workflows: Vec<PathBuf>,
}

/// A config file that shadows a scope's config via walk-up resolution.
#[derive(Debug)]
pub struct ShadowWarning {
    pub shadow_file: PathBuf,
    pub scope_root: PathBuf,
    pub affected_member: PathBuf,
    pub file_type: &'static str,
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

/// Build a `ProjectMap` from a `CrawlResult`.
pub fn build(root: &Path, crawl: &CrawlResult) -> ProjectMap {
    let rust_scopes = build_rust_scopes(root, crawl);
    let ts_scopes = build_ts_scopes(root, crawl);
    let root_configs = build_root_configs(root, crawl);
    let shadows = detect_shadows(root, &rust_scopes, crawl);

    ProjectMap {
        root: root.to_path_buf(),
        rust_scopes,
        ts_scopes,
        root_configs,
        shadows,
    }
}

// ---------------------------------------------------------------------------
// Rust scope building
// ---------------------------------------------------------------------------

fn build_rust_scopes(root: &Path, crawl: &CrawlResult) -> Vec<RustScope> {
    let mut scopes = Vec::new();
    let mut all_member_dirs: BTreeSet<PathBuf> = BTreeSet::new();

    // Pass 1: find all workspaces and resolve their members
    for cargo_path in &crawl.cargo_tomls {
        let Some(content) = crate::fs::read_file(cargo_path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };

        let Some(workspace) = table.get("workspace") else {
            continue; // Not a workspace — handle in pass 2
        };

        let ws_dir = cargo_path.parent().unwrap_or(root);
        let members = resolve_workspace_members(workspace, ws_dir, root);

        for m in &members {
            let _ = all_member_dirs.insert(m.dir.clone());
        }

        let configs = find_rust_configs_at(ws_dir, crawl);

        scopes.push(RustScope {
            root: relative_to(root, ws_dir),
            kind: RustScopeKind::Workspace,
            members,
            configs,
        });
    }

    // Pass 2: find standalone crates (package without workspace, not a member of any workspace)
    for cargo_path in &crawl.cargo_tomls {
        let Some(content) = crate::fs::read_file(cargo_path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };

        // Skip if it has [workspace] — already handled
        if table.get("workspace").is_some() {
            continue;
        }

        let Some(package) = table.get("package") else {
            continue; // Neither workspace nor package
        };

        let crate_dir = cargo_path.parent().unwrap_or(root);
        let rel_dir = relative_to(root, crate_dir);

        // Skip if this crate is a member of an already-found workspace
        if all_member_dirs.contains(&rel_dir) {
            continue;
        }

        let name = package
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_owned();

        let configs = find_rust_configs_at(crate_dir, crawl);

        scopes.push(RustScope {
            root: rel_dir.clone(),
            kind: RustScopeKind::StandaloneCrate,
            members: vec![RustMember { name, dir: rel_dir }],
            configs,
        });
    }

    scopes.sort_by(|a, b| a.root.cmp(&b.root));
    scopes
}

fn resolve_workspace_members(
    workspace: &toml::Value,
    ws_dir: &Path,
    project_root: &Path,
) -> Vec<RustMember> {
    let mut members = Vec::new();

    let Some(member_globs) = workspace.get("members").and_then(|m| m.as_array()) else {
        return members;
    };

    let excludes: BTreeSet<String> = workspace
        .get("exclude")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    for member_val in member_globs {
        let Some(member_str) = member_val.as_str() else {
            continue;
        };

        let pattern = ws_dir.join(member_str);
        let pattern_str = pattern.display().to_string();

        let Ok(paths) = glob::glob(&pattern_str) else {
            continue;
        };

        for member_path in paths.flatten() {
            if !member_path.join("Cargo.toml").exists() {
                continue;
            }

            // Check exclude
            if let Ok(rel) = member_path.strip_prefix(ws_dir) {
                let rel_str = rel.display().to_string();
                if excludes.contains(&rel_str) {
                    continue;
                }
            }

            let name = read_crate_name(&member_path);
            let dir = relative_to(project_root, &member_path);

            members.push(RustMember { name, dir });
        }
    }

    members
}

fn read_crate_name(path: &Path) -> String {
    let cargo_path = path.join("Cargo.toml");
    let Some(content) = crate::fs::read_file(&cargo_path) else {
        return path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_owned();
    };
    let Ok(table) = content.parse::<toml::Value>() else {
        return "unknown".to_owned();
    };
    table
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_owned()
}

/// Find which guardrail config files from the crawl exist at a given directory.
fn find_rust_configs_at(dir: &Path, crawl: &CrawlResult) -> RustScopeConfigs {
    RustScopeConfigs {
        clippy_toml: crawl
            .clippy_tomls
            .iter()
            .find(|p| p.parent() == Some(dir))
            .cloned(),
        deny_toml: crawl
            .deny_tomls
            .iter()
            .find(|p| p.parent() == Some(dir))
            .cloned(),
        rustfmt_toml: crawl
            .rustfmt_tomls
            .iter()
            .find(|p| p.parent() == Some(dir))
            .cloned(),
        cargo_lock: crawl
            .cargo_locks
            .iter()
            .find(|p| p.parent() == Some(dir))
            .cloned(),
        rust_toolchains: crawl
            .rust_toolchains
            .iter()
            .filter(|p| p.parent() == Some(dir))
            .cloned()
            .collect(),
        jscpd_config: crawl
            .jscpd_configs
            .iter()
            .find(|p| p.parent() == Some(dir))
            .cloned(),
    }
}

// ---------------------------------------------------------------------------
// TypeScript scope building
// ---------------------------------------------------------------------------

fn build_ts_scopes(root: &Path, crawl: &CrawlResult) -> Vec<TsScope> {
    let mut scopes = Vec::new();

    // Read pnpm-workspace.yaml for classification patterns
    let workspace_patterns = read_pnpm_patterns(crawl);

    for pkg_path in &crawl.package_jsons {
        let pkg_dir = pkg_path.parent().unwrap_or(root);

        // Skip root package.json — it's in root_configs
        if pkg_dir == root {
            continue;
        }

        let rel = relative_to(root, pkg_dir);
        let name = read_package_name(pkg_path);
        let kind = classify_ts_scope(&rel, &workspace_patterns);

        let configs = TsScopeConfigs {
            package_json: Some(pkg_path.clone()),
            tsconfig: crawl
                .tsconfigs
                .iter()
                .find(|p| p.parent() == Some(pkg_dir))
                .cloned(),
            eslint_config: crawl
                .eslint_configs
                .iter()
                .find(|p| p.parent() == Some(pkg_dir))
                .cloned(),
            stylelint_config: crawl
                .stylelint_configs
                .iter()
                .find(|p| p.parent() == Some(pkg_dir))
                .cloned(),
            velite_config: crawl
                .velite_configs
                .iter()
                .find(|p| p.parent() == Some(pkg_dir))
                .cloned(),
            next_config: crawl
                .next_configs
                .iter()
                .find(|p| p.parent() == Some(pkg_dir))
                .cloned(),
        };

        scopes.push(TsScope {
            path: rel,
            kind,
            name,
            configs,
        });
    }

    scopes.sort_by(|a, b| a.path.cmp(&b.path));
    scopes
}

#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection — internal tool, not user input
fn read_package_name(path: &Path) -> String {
    let Some(content) = crate::fs::read_file(path) else {
        return "unknown".to_owned();
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return "unknown".to_owned();
    };
    json.get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_owned()
}

fn read_pnpm_patterns(crawl: &CrawlResult) -> Vec<String> {
    let Some(path) = crawl.pnpm_workspaces.first() else {
        return Vec::new();
    };
    let Some(content) = crate::fs::read_file(path) else {
        return Vec::new();
    };
    // Simple YAML parsing — extract lines that look like: - "apps/*"
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim().strip_prefix("- ")?;
            let unquoted = trimmed.trim_matches('"').trim_matches('\'');
            Some(unquoted.to_owned())
        })
        .collect()
}

fn classify_ts_scope(rel_path: &Path, _patterns: &[String]) -> TsScopeKind {
    let path_str = rel_path.display().to_string();

    if path_str.starts_with("apps/") || path_str.starts_with("apps\\") {
        TsScopeKind::App
    } else if path_str.starts_with("packages/") || path_str.starts_with("packages\\") {
        TsScopeKind::Package
    } else {
        TsScopeKind::Tool
    }
}

// ---------------------------------------------------------------------------
// Root configs
// ---------------------------------------------------------------------------

fn build_root_configs(root: &Path, crawl: &CrawlResult) -> RootConfigs {
    let at_root = |paths: &[PathBuf]| -> Option<PathBuf> {
        paths.iter().find(|p| p.parent() == Some(root)).cloned()
    };

    RootConfigs {
        guardrail3_tomls: crawl.guardrail3_tomls.clone(),
        package_json: crawl
            .package_jsons
            .iter()
            .find(|p| p.parent() == Some(root))
            .cloned(),
        pnpm_workspaces: crawl.pnpm_workspaces.clone(),
        eslint_config: at_root(&crawl.eslint_configs),
        stylelint_config: at_root(&crawl.stylelint_configs),
        tsconfig_base: crawl
            .tsconfig_bases
            .iter()
            .find(|p| p.parent() == Some(root))
            .cloned(),
        npmrc: at_root(&crawl.npmrcs),
        jscpd_config: at_root(&crawl.jscpd_configs),
        cspell_config: at_root(&crawl.cspell_configs),
        prettier_config: at_root(&crawl.prettier_configs),
        rust_toolchains: crawl
            .rust_toolchains
            .iter()
            .filter(|p| p.parent() == Some(root))
            .cloned()
            .collect(),
        release_plz_tomls: crawl.release_plz_tomls.clone(),
        cliff_tomls: crawl.cliff_tomls.clone(),
        pre_commit_hooks: crawl.pre_commit_hooks.clone(),
        license_files: crawl.license_files.clone(),
        claude_mds: crawl.claude_mds.clone(),
        cargo_mutants_tomls: crawl.cargo_mutants_tomls.clone(),
        github_workflows: crawl.github_workflows.clone(),
    }
}

// ---------------------------------------------------------------------------
// Shadow detection
// ---------------------------------------------------------------------------

/// Find config files that sit between a crate and its workspace root,
/// shadowing the workspace-level config for that crate.
fn detect_shadows(
    root: &Path,
    rust_scopes: &[RustScope],
    crawl: &CrawlResult,
) -> Vec<ShadowWarning> {
    let mut warnings = Vec::new();

    for scope in rust_scopes {
        if matches!(scope.kind, RustScopeKind::StandaloneCrate) {
            continue; // No members to shadow
        }

        let scope_abs = root.join(&scope.root);

        for member in &scope.members {
            let member_abs = root.join(&member.dir);

            // Check walk-up configs: clippy.toml, rustfmt.toml
            // (deny.toml doesn't walk up — CWD only — so no shadowing)
            check_shadow_between(
                &member_abs,
                &scope_abs,
                &crawl.clippy_tomls,
                "clippy.toml",
                root,
                &scope.root,
                &member.dir,
                &mut warnings,
            );
            check_shadow_between(
                &member_abs,
                &scope_abs,
                &crawl.rustfmt_tomls,
                "rustfmt.toml",
                root,
                &scope.root,
                &member.dir,
                &mut warnings,
            );
        }
    }

    warnings
}

/// Check if any file in `all_files` sits between `from` (crate dir) and `to` (scope root),
/// exclusive of `to` itself (the scope root config is expected).
#[allow(clippy::too_many_arguments)] // reason: shadow detection needs from/to/files/type/root/scope/member/warnings context
fn check_shadow_between(
    from: &Path,
    to: &Path,
    all_files: &[PathBuf],
    file_type: &'static str,
    project_root: &Path,
    scope_root: &Path,
    member_dir: &Path,
    warnings: &mut Vec<ShadowWarning>,
) {
    for file in all_files {
        let Some(file_dir) = file.parent() else {
            continue;
        };

        // Skip the scope root itself — that's where the config SHOULD be
        if file_dir == to {
            continue;
        }

        // Check if this file is an ancestor of `from` and a descendant of `to`
        // (i.e., it's BETWEEN the crate and the workspace root)
        if from.starts_with(file_dir) && file_dir.starts_with(to) {
            warnings.push(ShadowWarning {
                shadow_file: relative_to(project_root, file),
                scope_root: scope_root.to_path_buf(),
                affected_member: member_dir.to_path_buf(),
                file_type,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn relative_to(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}
