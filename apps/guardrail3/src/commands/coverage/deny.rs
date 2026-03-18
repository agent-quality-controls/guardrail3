//! `deny.toml` coverage map.
//!
//! ## How `cargo-deny` finds its config (verified by testing):
//!
//! 1. If `--config <path>` is passed on the check subcommand, use that file
//! 2. Otherwise, look for `deny.toml` in the MANIFEST DIRECTORY (same dir as the
//!    `Cargo.toml` being analyzed, NOT necessarily CWD)
//! 3. No walk-up. No parent directory search. Manifest directory only.
//! 4. If not found, falls back to default config with a warning:
//!    "unable to find a config path, falling back to default config"
//! 5. Exceptions file: also checks `<manifest-dir>/deny.exceptions.toml`,
//!    `<manifest-dir>/.deny.exceptions.toml`, `<manifest-dir>/.cargo/deny.exceptions.toml`
//!
//! Key behaviors:
//! - `cargo deny --manifest-path apps/validator-rust/Cargo.toml check` looks for
//!   `apps/validator-rust/deny.toml` regardless of CWD
//! - Each workspace needs its own `deny.toml` co-located with its `Cargo.toml`
//! - No shadowing possible — one `deny.toml` per workspace, no walk-up
//!
//! Verified:
//! - `cd apps/validator-rust && cargo deny check` → finds `apps/validator-rust/deny.toml`
//! - `cargo deny --manifest-path apps/validator-rust/Cargo.toml check` from root → also finds it
//! - `cargo deny --manifest-path Cargo.toml check` from root → warns "unable to find config"
//!
//! Sources:
//! - <https://embarkstudios.github.io/cargo-deny/cli/check.html>
//! - <https://embarkstudios.github.io/cargo-deny/checks/cfg.html>

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::app::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Data model (same structure as clippy, different details)
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct DenyCoverage {
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
        deny_toml: Option<DenyTomlInfo>,
        crates: Vec<CrateNode>,
    },
    #[serde(rename = "package")]
    Package {
        name: String,
        path: String,
        cargo_toml: String,
        deny_toml: Option<DenyTomlInfo>,
        covered_by: Option<String>,
    },
}

/// Crate node in deny coverage.
///
/// `covered_by` is derived from the workspace's `deny_toml` —
/// `cargo-deny` runs per-workspace, so all members of a workspace
/// with `deny.toml` are covered. No per-crate walk-up resolution.
#[derive(Serialize)]
pub struct CrateNode {
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub cargo_toml: String,
    pub covered_by: Option<String>,
}

#[derive(Serialize)]
pub struct DenyTomlInfo {
    pub path: String,
    pub bans: usize,
    pub advisory_ignores: usize,
}

#[derive(Serialize)]
pub struct Summary {
    pub total_crates: u32,
    pub covered: u32,
    pub uncovered: u32,
}

// ---------------------------------------------------------------------------
// Build (reuses clippy's workspace/package discovery, different coverage logic)
// ---------------------------------------------------------------------------

/// Build deny.toml coverage data.
///
/// Coverage for deny.toml is simpler than clippy: no walk-up, no shadowing.
/// A crate is "covered" if its workspace root (where `cargo deny` would be run)
/// has a `deny.toml`. For standalone packages, the package dir needs `deny.toml`.
#[allow(clippy::too_many_lines)] // reason: two-pass workspace+package discovery
pub fn build(root: &Path, crawl: &CrawlResult) -> DenyCoverage {
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
        let deny_info = parse_deny_toml(dir, &crawl.deny_tomls, root);

        // deny.toml at workspace root covers all members (no walk-up — CWD resolution)
        let deny_path = deny_info.as_ref().map(|d| d.path.clone());

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

                    let _ = all_member_dirs.insert(member_path);

                    crate_nodes.push(CrateNode {
                        kind: "package",
                        name,
                        cargo_toml: format!("{crate_rel}/Cargo.toml"),
                        path: crate_rel,
                        covered_by: deny_path.clone(),
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
            deny_toml: deny_info,
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
        let deny_info = parse_deny_toml(dir, &crawl.deny_tomls, root);
        let covered = deny_info.as_ref().map(|d| d.path.clone());

        let _ = all_member_dirs.insert(dir.to_path_buf());

        scopes.push(Scope::Package {
            name,
            cargo_toml: format!("{rel_dir}/Cargo.toml").replace("./", ""),
            path: rel_dir,
            deny_toml: deny_info,
            covered_by: covered,
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

    for scope in &scopes {
        match scope {
            Scope::Workspace { crates, .. } => {
                for c in crates {
                    total = total.saturating_add(1);
                    if c.covered_by.is_some() {
                        covered = covered.saturating_add(1);
                    }
                }
            }
            Scope::Package { covered_by, .. } => {
                total = total.saturating_add(1);
                if covered_by.is_some() {
                    covered = covered.saturating_add(1);
                }
            }
        }
    }

    DenyCoverage {
        tool: "deny",
        resolution: "manifest directory (co-located with Cargo.toml, no walk-up)",
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

    println!("deny.toml coverage");
    println!("(cargo-deny looks in manifest directory — co-located with Cargo.toml, no walk-up)\n");

    for scope in &data.scopes {
        match scope {
            Scope::Workspace {
                path,
                excludes,
                deny_toml,
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
                match deny_toml {
                    Some(info) => {
                        println!(
                            "  deny.toml                {} bans, {} advisory ignores",
                            info.bans, info.advisory_ignores
                        );
                    }
                    None => println!("  deny.toml                MISSING"),
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
                    if c.covered_by.is_none() {
                        println!("{indent}  ⚠ UNCOVERED");
                    }
                }
                println!();
            }
            Scope::Package {
                name,
                path,
                deny_toml,
                covered_by,
                ..
            } => {
                println!("{path}/");
                println!("  Cargo.toml               [package] {name}");
                match deny_toml {
                    Some(info) => {
                        println!(
                            "  deny.toml                {} bans, {} advisory ignores",
                            info.bans, info.advisory_ignores
                        );
                    }
                    None => println!("  deny.toml                MISSING"),
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

fn parse_deny_toml(dir: &Path, deny_tomls: &[PathBuf], root: &Path) -> Option<DenyTomlInfo> {
    let path = deny_tomls.iter().find(|p| p.parent() == Some(dir))?;
    let content = crate::fs::read_file(path)?;
    let table = content.parse::<toml::Value>().ok()?;
    let bans = table
        .get("bans")
        .and_then(|b| b.get("deny"))
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    let advisory_ignores = table
        .get("advisories")
        .and_then(|a| a.get("ignore"))
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    Some(DenyTomlInfo {
        path: rel_str(root, path),
        bans,
        advisory_ignores,
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
