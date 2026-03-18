//! `rustfmt.toml` coverage map.
//!
//! ## How rustfmt finds its config (verified from rustfmt docs + source):
//!
//! 1. Walk UP parent directories from the file being formatted
//! 2. Check each directory for `rustfmt.toml` or `.rustfmt.toml`
//! 3. Stop at the first one found (nearest wins, shadows completely)
//! 4. If none found, check `$HOME` and `$XDG_CONFIG_HOME/rustfmt/`
//! 5. If still nothing, use defaults
//!
//! Key behaviors:
//! - Walks up from the FILE being formatted, not from `Cargo.toml` dir
//! - In practice with `cargo fmt`, it formats files in each crate, so the
//!   walk-up starts from each source file's directory
//! - A `rustfmt.toml` in a subdirectory COMPLETELY SHADOWS the parent — no merging
//! - Formatting should be uniform across a project, so per-crate `rustfmt.toml` is
//!   almost always a mistake
//!
//! Sources:
//! - <https://rust-lang.github.io/rustfmt/>
//! - <https://github.com/rust-lang/rustfmt>

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::app::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct RustfmtCoverage {
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
        rustfmt_toml: Option<RustfmtTomlInfo>,
        crates: Vec<CrateNode>,
    },
    #[serde(rename = "package")]
    Package {
        name: String,
        path: String,
        cargo_toml: String,
        rustfmt_toml: Option<RustfmtTomlInfo>,
        covered_by: Option<String>,
        shadows: bool,
    },
}

#[derive(Serialize)]
pub struct CrateNode {
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub cargo_toml: String,
    pub covered_by: Option<String>,
    pub shadows: bool,
}

#[derive(Serialize)]
pub struct RustfmtTomlInfo {
    pub path: String,
    pub settings: usize,
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

#[allow(clippy::too_many_lines)] // reason: two-pass workspace+package discovery
pub fn build(root: &Path, crawl: &CrawlResult) -> RustfmtCoverage {
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
        let fmt_info = parse_rustfmt_toml(dir, &crawl.rustfmt_tomls, root);

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
                    let covered = find_covering_rustfmt(&member_path, root, &crawl.rustfmt_tomls)
                        .map(|p| format!("{}/rustfmt.toml", rel_str(root, &p)));
                    let shadows = has_own_rustfmt(&member_path, dir, &crawl.rustfmt_tomls);

                    let _ = all_member_dirs.insert(member_path);

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

        let members_full: Vec<String> = crate_nodes.iter().map(|c| c.path.clone()).collect();

        scopes.push(Scope::Workspace {
            cargo_toml: format!("{rel_dir}/Cargo.toml").replace("./", ""),
            path: rel_dir,
            members: members_full,
            excludes,
            rustfmt_toml: fmt_info,
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
        if table.get("workspace").is_some() || table.get("package").is_none() {
            continue;
        }

        let dir = cargo_path.parent().unwrap_or(root);
        if all_member_dirs.contains(dir) {
            continue;
        }

        let rel_dir = rel_str(root, dir);
        let name = read_package_name(dir);
        let fmt_info = parse_rustfmt_toml(dir, &crawl.rustfmt_tomls, root);
        let covered = find_covering_rustfmt(dir, root, &crawl.rustfmt_tomls)
            .map(|p| format!("{}/rustfmt.toml", rel_str(root, &p)));

        let _ = all_member_dirs.insert(dir.to_path_buf());

        scopes.push(Scope::Package {
            name,
            cargo_toml: format!("{rel_dir}/Cargo.toml").replace("./", ""),
            path: rel_dir,
            rustfmt_toml: fmt_info,
            covered_by: covered,
            shadows: false,
        });
    }

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

    RustfmtCoverage {
        tool: "rustfmt",
        resolution: "walk-up from file being formatted",
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

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_json(root: &Path, crawl: &CrawlResult) {
    let data = build(root, crawl);
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        println!("{json}");
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
#[allow(clippy::too_many_lines)] // reason: tree rendering
pub fn print_tree(root: &Path, crawl: &CrawlResult) {
    let data = build(root, crawl);

    println!("rustfmt.toml coverage");
    println!(
        "(rustfmt walks UP from each source file — nearest rustfmt.toml wins, shadows completely)\n"
    );

    for scope in &data.scopes {
        match scope {
            Scope::Workspace {
                path,
                excludes,
                rustfmt_toml,
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
                match rustfmt_toml {
                    Some(info) => println!("  rustfmt.toml             {} settings", info.settings),
                    None => println!("  rustfmt.toml             MISSING"),
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

                    let member_rel = c
                        .path
                        .strip_prefix(path)
                        .and_then(|s| s.strip_prefix('/'))
                        .unwrap_or(&c.path);
                    println!("{prefix}{member_rel}/");
                    println!("{indent}  Cargo.toml             [package] {}", c.name);
                    if c.shadows {
                        println!(
                            "{indent}  rustfmt.toml           ⚠ SHADOWS workspace rustfmt.toml"
                        );
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
                rustfmt_toml,
                covered_by,
                ..
            } => {
                println!("{path}/");
                println!("  Cargo.toml               [package] {name}");
                match rustfmt_toml {
                    Some(info) => println!("  rustfmt.toml             {} settings", info.settings),
                    None => println!("  rustfmt.toml             MISSING"),
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

/// Walk-up resolution: same as clippy.
fn find_covering_rustfmt(
    crate_dir: &Path,
    project_root: &Path,
    rustfmt_tomls: &[PathBuf],
) -> Option<PathBuf> {
    let mut current = crate_dir;
    loop {
        if rustfmt_tomls.iter().any(|p| p.parent() == Some(current)) {
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

fn has_own_rustfmt(crate_dir: &Path, workspace_dir: &Path, rustfmt_tomls: &[PathBuf]) -> bool {
    if crate_dir == workspace_dir {
        return false;
    }
    rustfmt_tomls.iter().any(|p| p.parent() == Some(crate_dir))
}

fn parse_rustfmt_toml(
    dir: &Path,
    rustfmt_tomls: &[PathBuf],
    root: &Path,
) -> Option<RustfmtTomlInfo> {
    let path = rustfmt_tomls.iter().find(|p| p.parent() == Some(dir))?;
    let content = crate::fs::read_file(path)?;
    // Count non-empty, non-comment lines as "settings"
    let settings = content
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .count();
    Some(RustfmtTomlInfo {
        path: rel_str(root, path),
        settings,
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
