//! `clippy.toml` coverage map.
//!
//! ## How clippy finds its config (verified from clippy source + docs):
//!
//! 1. Check `$CLIPPY_CONF_DIR` env var (if set)
//! 2. Check `$CARGO_MANIFEST_DIR` env var (set by cargo to the crate's `Cargo.toml` dir)
//! 3. Walk UP parent directories from step 2, checking each for `clippy.toml` or `.clippy.toml`
//! 4. If nothing found, check `$HOME` and `$XDG_CONFIG_HOME/clippy/`
//! 5. If still nothing, use defaults (no bans, default thresholds)
//!
//! Key behaviors:
//! - When `cargo clippy --workspace` runs, `CARGO_MANIFEST_DIR` is set PER CRATE being compiled
//! - A `clippy.toml` in a subdirectory COMPLETELY SHADOWS the parent — no merging
//! - Per-crate `clippy.toml` is NOT officially supported (issue #7353) but the walk-up
//!   means a file placed in a crate's dir WILL be found and used, shadowing the workspace one
//!
//! Sources:
//! - <https://doc.rust-lang.org/clippy/configuration.html>
//! - <https://github.com/rust-lang/rust-clippy/issues/7353>
//! - <https://github.com/rust-lang/rust-clippy/blob/master/clippy_config/src/conf.rs>

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::app::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct ClippyCoverage {
    pub tool: &'static str,
    pub resolution: &'static str,
    pub project: String,
    pub scopes: Vec<Scope>,
    pub summary: Summary,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub enum Scope {
    #[serde(rename = "workspace")]
    Workspace {
        path: String,
        cargo_toml: String,
        members: Vec<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        excludes: Vec<String>,
        clippy_toml: Option<ClippyTomlInfo>,
        crates: Vec<CrateNode>,
    },
    #[serde(rename = "package")]
    Package {
        name: String,
        path: String,
        cargo_toml: String,
        clippy_toml: Option<ClippyTomlInfo>,
        covered_by: Option<String>,
        shadows: bool,
    },
}

#[derive(Serialize)]
pub struct CrateNode {
    pub kind: &'static str, // always "package"
    pub name: String,
    pub path: String,
    pub cargo_toml: String,
    pub covered_by: Option<String>,
    pub shadows: bool,
}

#[derive(Serialize)]
pub struct ClippyTomlInfo {
    pub path: String,
    pub methods: usize,
    pub types: usize,
}

#[derive(Serialize)]
pub struct Summary {
    pub total_crates: u32,
    pub covered: u32,
    pub uncovered: u32,
    pub shadowed: u32,
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

/// Build the clippy coverage data from crawl results.
#[allow(clippy::too_many_lines)] // reason: builds full project structure in two passes — splitting would obscure the flow
pub fn build(root: &Path, crawl: &CrawlResult) -> ClippyCoverage {
    let mut scopes = Vec::new();
    let mut all_member_dirs: std::collections::BTreeSet<PathBuf> =
        std::collections::BTreeSet::new();

    // Pass 1: workspaces
    for cargo_path in &crawl.cargo_tomls {
        let Some(content) = crate::fs::read_file(cargo_path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };
        let Some(ws) = table.get("workspace") else {
            continue;
        };

        let dir = cargo_path.parent().unwrap_or(root);
        let rel_dir = rel_str(root, dir);

        let excludes = parse_string_array(ws.get("exclude"));
        // Resolve member globs to actual crate directories
        let mut crate_nodes = Vec::new();
        if let Some(members_arr) = ws.get("members").and_then(|m| m.as_array()) {
            for member_val in members_arr {
                let Some(pattern_str) = member_val.as_str() else {
                    continue;
                };
                let pattern = dir.join(pattern_str);
                let Ok(paths) = glob::glob(&pattern.display().to_string()) else {
                    continue;
                };
                for member_path in paths.flatten() {
                    if !member_path.join("Cargo.toml").exists() {
                        continue;
                    }
                    if let Ok(ws_rel) = member_path.strip_prefix(dir) {
                        if excludes.contains(&ws_rel.display().to_string()) {
                            continue;
                        }
                    }
                    let crate_rel = rel_str(root, &member_path);
                    let name = read_package_name(&member_path);
                    let covered = find_covering_clippy(&member_path, root, &crawl.clippy_tomls)
                        .map(|p| format!("{}/clippy.toml", rel_str(root, &p)));
                    let shadows = has_own_clippy(&member_path, dir, &crawl.clippy_tomls);

                    let _ = all_member_dirs.insert(member_path.clone());

                    crate_nodes.push(CrateNode {
                        kind: "package",
                        name,
                        cargo_toml: format!("{crate_rel}/Cargo.toml"),
                        path: crate_rel,
                        covered_by: covered,
                        shadows,
                    });
                }
            }
        }

        // Resolve members to project-root-relative paths for the JSON output
        let members_full: Vec<String> = crate_nodes.iter().map(|c| c.path.clone()).collect();

        let clippy_info = parse_clippy_toml(dir, &crawl.clippy_tomls, root);

        scopes.push(Scope::Workspace {
            cargo_toml: format!("{rel_dir}/Cargo.toml").replace("./", ""),
            path: rel_dir,
            members: members_full,
            excludes,
            clippy_toml: clippy_info,
            crates: crate_nodes,
        });
    }

    // Pass 2: independent packages
    for cargo_path in &crawl.cargo_tomls {
        let Some(content) = crate::fs::read_file(cargo_path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };
        if table.get("workspace").is_some() {
            continue;
        }
        if table.get("package").is_none() {
            continue;
        }

        let dir = cargo_path.parent().unwrap_or(root);
        let rel_dir_path = root.join("").join(dir.strip_prefix(root).unwrap_or(dir));

        if all_member_dirs.contains(dir) {
            continue;
        }

        let rel_dir = rel_str(root, dir);
        let name = read_package_name(dir);
        let clippy_info = parse_clippy_toml(dir, &crawl.clippy_tomls, root);
        let covered = find_covering_clippy(dir, root, &crawl.clippy_tomls)
            .map(|p| format!("{}/clippy.toml", rel_str(root, &p)));

        let _ = all_member_dirs.insert(rel_dir_path);

        scopes.push(Scope::Package {
            name,
            cargo_toml: format!("{rel_dir}/Cargo.toml").replace("./", ""),
            path: rel_dir,
            clippy_toml: clippy_info,
            covered_by: covered,
            shadows: false,
        });
    }

    // Sort scopes by path
    scopes.sort_by(|a, b| {
        let path_a = match a {
            Scope::Workspace { path, .. } | Scope::Package { path, .. } => path,
        };
        let path_b = match b {
            Scope::Workspace { path, .. } | Scope::Package { path, .. } => path,
        };
        path_a.cmp(path_b)
    });

    // Summary
    let mut total: u32 = 0;
    let mut covered: u32 = 0;
    let mut shadowed: u32 = 0;

    for scope in &scopes {
        match scope {
            Scope::Workspace { crates, .. } => {
                for c in crates {
                    total = total.saturating_add(1);
                    if c.covered_by.is_some() {
                        covered = covered.saturating_add(1);
                    }
                    if c.shadows {
                        shadowed = shadowed.saturating_add(1);
                    }
                }
            }
            Scope::Package {
                covered_by,
                shadows,
                ..
            } => {
                total = total.saturating_add(1);
                if covered_by.is_some() {
                    covered = covered.saturating_add(1);
                }
                if *shadows {
                    shadowed = shadowed.saturating_add(1);
                }
            }
        }
    }

    ClippyCoverage {
        tool: "clippy",
        resolution: "walk-up from CARGO_MANIFEST_DIR",
        project: root
            .canonicalize()
            .unwrap_or_else(|_| root.to_path_buf())
            .display()
            .to_string(),
        scopes,
        summary: Summary {
            total_crates: total,
            covered,
            uncovered: total.saturating_sub(covered),
            shadowed,
        },
    }
}

// ---------------------------------------------------------------------------
// Print
// ---------------------------------------------------------------------------

/// Print as JSON.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
pub fn print_json(root: &Path, crawl: &CrawlResult) {
    let data = build(root, crawl);
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        println!("{json}");
    }
}

/// Print as human-readable tree.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
#[allow(clippy::too_many_lines)] // reason: tree rendering with multiple node types
pub fn print_tree(root: &Path, crawl: &CrawlResult) {
    let data = build(root, crawl);

    println!("clippy.toml coverage");
    println!(
        "(clippy walks UP from each crate dir — nearest clippy.toml wins, shadows completely)\n"
    );

    for scope in &data.scopes {
        match scope {
            Scope::Workspace {
                path,
                excludes,
                clippy_toml,
                crates,
                ..
            } => {
                let display = if path.is_empty() { "." } else { path.as_str() };
                let member_count = crates.len();
                println!("{display}/");
                println!("  Cargo.toml               [workspace] members={member_count}");
                if !excludes.is_empty() {
                    println!(
                        "                           excludes: {}",
                        excludes.join(", ")
                    );
                }
                match clippy_toml {
                    Some(info) => {
                        println!(
                            "  clippy.toml              {} methods, {} types",
                            info.methods, info.types
                        );
                    }
                    None => println!("  clippy.toml              MISSING"),
                }

                let count = crates.len();
                for (i, c) in crates.iter().enumerate() {
                    let is_last = i == count.saturating_sub(1);
                    let prefix = if is_last {
                        "  └── "
                    } else {
                        "  ├── "
                    };
                    let indent = if is_last { "      " } else { "  │   " };

                    // Show path relative to workspace
                    let member_rel = c
                        .path
                        .strip_prefix(path)
                        .and_then(|s| s.strip_prefix('/'))
                        .unwrap_or(&c.path);
                    println!("{prefix}{member_rel}/");
                    println!("{indent}  Cargo.toml             [package] {}", c.name);

                    if c.shadows {
                        println!("{indent}  clippy.toml           ⚠ SHADOWS workspace clippy.toml");
                    }
                    if c.covered_by.is_none() {
                        println!("{indent}  ⚠ UNCOVERED");
                    }
                }
                println!();
            }
            Scope::Package {
                name,
                path,
                clippy_toml,
                covered_by,
                ..
            } => {
                println!("{path}/");
                println!("  Cargo.toml               [package] {name}");
                match clippy_toml {
                    Some(info) => {
                        println!(
                            "  clippy.toml              {} methods, {} types",
                            info.methods, info.types
                        );
                    }
                    None => println!("  clippy.toml              MISSING"),
                }
                if covered_by.is_none() {
                    println!("  ⚠ UNCOVERED");
                }
                println!();
            }
        }
    }

    println!(
        "Summary: {}/{} crates covered, {} uncovered",
        data.summary.covered, data.summary.total_crates, data.summary.uncovered
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Simulate clippy's walk-up: from `crate_dir`, walk up looking for `clippy.toml`.
fn find_covering_clippy(
    crate_dir: &Path,
    project_root: &Path,
    clippy_tomls: &[PathBuf],
) -> Option<PathBuf> {
    let mut current = crate_dir;
    loop {
        if clippy_tomls.iter().any(|p| p.parent() == Some(current)) {
            return Some(current.to_path_buf());
        }
        if current == project_root {
            return None;
        }
        current = match current.parent() {
            Some(p) if !p.as_os_str().is_empty() => p,
            _ => return None,
        };
    }
}

/// Check if a crate has its own `clippy.toml` that shadows the workspace one.
fn has_own_clippy(crate_dir: &Path, workspace_dir: &Path, clippy_tomls: &[PathBuf]) -> bool {
    if crate_dir == workspace_dir {
        return false;
    }
    clippy_tomls.iter().any(|p| p.parent() == Some(crate_dir))
}

fn parse_clippy_toml(dir: &Path, clippy_tomls: &[PathBuf], root: &Path) -> Option<ClippyTomlInfo> {
    let path = clippy_tomls.iter().find(|p| p.parent() == Some(dir))?;
    let content = crate::fs::read_file(path)?;
    let table = content.parse::<toml::Value>().ok()?;
    let methods = table
        .get("disallowed-methods")
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    let types = table
        .get("disallowed-types")
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    Some(ClippyTomlInfo {
        path: rel_str(root, path),
        methods,
        types,
    })
}

fn read_package_name(dir: &Path) -> String {
    let cargo_path = dir.join("Cargo.toml");
    let Some(content) = crate::fs::read_file(&cargo_path) else {
        return dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_owned();
    };
    let Ok(table) = content.parse::<toml::Value>() else {
        return "?".to_owned();
    };
    table
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("?")
        .to_owned()
}

fn parse_string_array(val: Option<&toml::Value>) -> Vec<String> {
    val.and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn rel_str(root: &Path, path: &Path) -> String {
    let s = path
        .strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string();
    if s.is_empty() { ".".to_owned() } else { s }
}
