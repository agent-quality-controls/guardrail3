//! `clippy.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from `CARGO_MANIFEST_DIR`
//!
//! Checks `clippy.toml` and `.clippy.toml` at each parent directory.
//! Nearest wins, shadows completely. No merging.
//!
//! Verified from clippy source: `clippy_config/src/conf.rs` `lookup_conf_file()`

use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool, Target};

pub struct ClippyCoverage;

impl CoverageTool for ClippyCoverage {
    fn name(&self) -> &'static str {
        "clippy"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CARGO_MANIFEST_DIR — nearest clippy.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.clippy_tomls
    }

    fn targets(&self, crawl: &CrawlResult, root: &Path) -> Vec<Target> {
        rust_crate_targets(crawl, root)
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            return serde_json::json!({"error": "parse error"});
        };
        let methods = table
            .get("disallowed-methods")
            .and_then(|v| v.as_array())
            .map_or(0, Vec::len);
        let types = table
            .get("disallowed-types")
            .and_then(|v| v.as_array())
            .map_or(0, Vec::len);
        serde_json::json!({"methods": methods, "types": types})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

/// Print clippy coverage (delegates to engine).
#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_json(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&ClippyCoverage, root, crawl);
    engine::print_json(&map);
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_tree(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&ClippyCoverage, root, crawl);
    engine::print_tree(&map);
}

// ---------------------------------------------------------------------------
// Shared: extract Rust crate targets from crawl
// (reused by deny.rs, rustfmt.rs)
// ---------------------------------------------------------------------------

/// Build targets from all Cargo.toml files that have `[package]`.
/// Each package crate is a target. Groups them by workspace membership.
pub(crate) fn rust_crate_targets(crawl: &CrawlResult, root: &Path) -> Vec<Target> {
    let mut targets = Vec::new();
    let mut workspace_members: std::collections::BTreeMap<PathBuf, PathBuf> =
        std::collections::BTreeMap::new();

    // Pass 1: find workspaces and resolve members
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

        let ws_dir = cargo_path.parent().unwrap_or(root);
        let ws_rel = rel(root, ws_dir);

        let excludes: std::collections::BTreeSet<String> = ws
            .get("exclude")
            .and_then(|e| e.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();

        if let Some(members_arr) = ws.get("members").and_then(|m| m.as_array()) {
            for member_val in members_arr {
                let Some(pattern_str) = member_val.as_str() else {
                    continue;
                };
                let pattern = ws_dir.join(pattern_str);
                let Ok(paths) = glob::glob(&pattern.display().to_string()) else {
                    continue;
                };
                for member_path in paths.flatten() {
                    if !member_path.join("Cargo.toml").exists() {
                        continue;
                    }
                    if let Ok(ws_local) = member_path.strip_prefix(ws_dir) {
                        if excludes.contains(&ws_local.display().to_string()) {
                            continue;
                        }
                    }
                    let _ = workspace_members.insert(member_path.clone(), ws_rel.clone());
                }
            }
        }
    }

    // Pass 2: build targets from all [package] Cargo.tomls
    for cargo_path in &crawl.cargo_tomls {
        let Some(content) = crate::fs::read_file(cargo_path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };
        let Some(pkg) = table.get("package") else {
            continue;
        };

        let crate_dir = cargo_path.parent().unwrap_or(root);
        let crate_rel = rel(root, crate_dir);
        let name = pkg
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("?")
            .to_owned();

        let group = workspace_members.get(crate_dir).cloned();

        targets.push(Target {
            name,
            structure_file: crate_rel.join("Cargo.toml"),
            path: crate_rel,
            structure_kind: "package",
            group,
        });
    }

    targets.sort_by(|a, b| a.path.cmp(&b.path));
    targets
}

fn rel(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}
