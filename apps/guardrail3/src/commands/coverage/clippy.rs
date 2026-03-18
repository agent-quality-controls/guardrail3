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

use crate::app::crawl::CrawlResult;

/// Print clippy.toml coverage as a hierarchical tree.
///
/// Shows every workspace/package with its Cargo.toml, and whether
/// each crate is covered by a clippy.toml via walk-up resolution.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
#[allow(clippy::too_many_lines)] // reason: tree rendering with multiple node types
pub fn print(root: &Path, crawl: &CrawlResult) {
    println!("clippy.toml coverage");
    println!(
        "(clippy walks UP from each crate dir — nearest clippy.toml wins, shadows completely)\n"
    );

    // Parse all Cargo.tomls into a structure
    let mut workspaces: Vec<WorkspaceNode> = Vec::new();
    let mut all_member_dirs: std::collections::BTreeSet<PathBuf> =
        std::collections::BTreeSet::new();

    // Pass 1: find workspaces and their members
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
        let rel_dir = rel(root, dir);
        let has_package = table.get("package").is_some();

        let members_arr = ws.get("members").and_then(|m| m.as_array());
        let excludes: Vec<String> = ws
            .get("exclude")
            .and_then(|e| e.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();

        // Resolve member globs
        let mut members = Vec::new();
        if let Some(arr) = members_arr {
            for member_val in arr {
                let Some(pattern_str) = member_val.as_str() else {
                    continue;
                };
                let pattern = dir.join(pattern_str);
                if let Ok(paths) = glob::glob(&pattern.display().to_string()) {
                    for member_path in paths.flatten() {
                        if !member_path.join("Cargo.toml").exists() {
                            continue;
                        }
                        let member_rel = rel(root, &member_path);
                        // Check exclude
                        if let Ok(ws_rel) = member_path.strip_prefix(dir) {
                            if excludes.contains(&ws_rel.display().to_string()) {
                                continue;
                            }
                        }
                        let name = read_package_name(&member_path);
                        let covered = find_covering_clippy(&member_path, root, &crawl.clippy_tomls);
                        let has_own_clippy = crawl
                            .clippy_tomls
                            .iter()
                            .any(|p| p.parent() == Some(member_path.as_path()));

                        let _ = all_member_dirs.insert(member_rel.clone());
                        members.push(MemberNode {
                            name,
                            dir: member_rel,
                            covered_by: covered.map(|p| rel(root, &p)),
                            has_own_clippy,
                        });
                    }
                }
            }
        }

        let clippy_info = clippy_summary(dir, &crawl.clippy_tomls);

        workspaces.push(WorkspaceNode {
            dir: rel_dir,
            has_package,
            members,
            excludes,
            clippy: clippy_info,
        });
    }

    // Pass 2: find independent packages (not a member of any workspace)
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
        let Some(_pkg) = table.get("package") else {
            continue;
        };

        let dir = cargo_path.parent().unwrap_or(root);
        let rel_dir = rel(root, dir);

        if all_member_dirs.contains(&rel_dir) {
            continue; // It's a workspace member, already shown
        }

        let clippy_info = clippy_summary(dir, &crawl.clippy_tomls);

        // Show as an independent package (no separate members)
        workspaces.push(WorkspaceNode {
            dir: rel_dir.clone(),
            has_package: true,
            members: Vec::new(), // No separate members — it IS the member
            excludes: Vec::new(),
            clippy: clippy_info,
        });

        // But we still need coverage info for the summary
        // Store it as a pseudo-member for counting
        let _ = all_member_dirs.insert(rel_dir);
    }

    workspaces.sort_by(|a, b| a.dir.cmp(&b.dir));

    // Render tree
    let mut total_crates = 0u32;
    let mut covered_crates = 0u32;

    for ws in &workspaces {
        let dir_display = if ws.dir.as_os_str().is_empty() {
            ".".to_owned()
        } else {
            ws.dir.display().to_string()
        };

        // Workspace/package header
        if ws.members.is_empty() && ws.has_package {
            // Independent package
            println!("{dir_display}/");
            println!(
                "  Cargo.toml               [package] {}",
                read_package_name_from_rel(root, &ws.dir)
            );
            match &ws.clippy {
                Some(info) => println!("  clippy.toml              {info}"),
                None => println!("  clippy.toml              MISSING"),
            }
            // Coverage
            let covered = find_covering_clippy(&root.join(&ws.dir), root, &crawl.clippy_tomls);
            total_crates = total_crates.saturating_add(1);
            if covered.is_some() {
                covered_crates = covered_crates.saturating_add(1);
            } else {
                println!("  ⚠ UNCOVERED");
            }
        } else {
            // Workspace
            let member_count = ws.members.len();
            println!("{dir_display}/");
            if ws.has_package {
                println!("  Cargo.toml               [workspace+package] members={member_count}");
            } else {
                println!("  Cargo.toml               [workspace] members={member_count}");
            }
            if !ws.excludes.is_empty() {
                println!(
                    "                           excludes: {}",
                    ws.excludes.join(", ")
                );
            }
            match &ws.clippy {
                Some(info) => println!("  clippy.toml              {info}"),
                None => println!("  clippy.toml              MISSING"),
            }

            // Members
            for (i, member) in ws.members.iter().enumerate() {
                let is_last = i == member_count.saturating_sub(1);
                let prefix = if is_last {
                    "  └── "
                } else {
                    "  ├── "
                };
                let cont = if is_last { "      " } else { "  │   " };

                // Show member path relative to workspace, not project root
                let member_rel = member.dir.strip_prefix(&ws.dir).unwrap_or(&member.dir);
                println!("{prefix}{}/", member_rel.display());
                println!("{cont}  Cargo.toml             [package] {}", member.name);

                if member.has_own_clippy {
                    println!("{cont}  clippy.toml           ⚠ SHADOWS workspace clippy.toml");
                }

                total_crates = total_crates.saturating_add(1);
                if member.covered_by.is_some() {
                    covered_crates = covered_crates.saturating_add(1);
                } else {
                    println!("{cont}  ⚠ UNCOVERED");
                }
            }
        }
        println!();
    }

    println!(
        "Summary: {covered_crates}/{total_crates} crates covered, {} uncovered",
        total_crates.saturating_sub(covered_crates)
    );
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

struct WorkspaceNode {
    dir: PathBuf,
    has_package: bool,
    members: Vec<MemberNode>,
    excludes: Vec<String>,
    clippy: Option<String>,
}

struct MemberNode {
    name: String,
    dir: PathBuf,
    covered_by: Option<PathBuf>,
    has_own_clippy: bool,
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/// Simulate clippy's walk-up: from `crate_dir`, walk up parent directories
/// looking for `clippy.toml` or `.clippy.toml`. Return the directory that has it.
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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn rel(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
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

fn read_package_name_from_rel(root: &Path, rel_path: &Path) -> String {
    read_package_name(&root.join(rel_path))
}

fn clippy_summary(dir: &Path, clippy_tomls: &[PathBuf]) -> Option<String> {
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
    Some(format!("{methods} methods, {types} types"))
}
