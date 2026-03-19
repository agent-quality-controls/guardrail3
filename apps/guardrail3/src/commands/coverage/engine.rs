//! Generic coverage map engine.
//!
//! Shared walk-up resolution, data model, and rendering for all config file types.
//! Each tool module implements `CoverageTool` and plugs into this engine.
//!
//! Coverage is DIRECTORY-BASED, not file-based or crate-based.
//! Every directory with source files is a coverage target.
//! Uncovered directories are collapsed to top-level ancestors.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::app::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Tool trait — each config file type implements this
// ---------------------------------------------------------------------------

/// What a tool-specific coverage module provides.
pub trait CoverageTool {
    /// Tool name (e.g., "clippy", "deny", "eslint").
    fn name(&self) -> &'static str;

    /// How the tool finds its config (shown in output header).
    fn resolution_description(&self) -> &'static str;

    /// Extract all instances of this config file from the crawl result.
    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf];

    /// All directories that contain source files this tool would check.
    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf>;

    /// Parse tool-specific details from a config file.
    fn parse_details(&self, config_path: &Path) -> serde_json::Value;

    /// Whether this tool walks up parent directories.
    fn walks_up(&self) -> bool;
}

// ---------------------------------------------------------------------------
// Output data model
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct CoverageMap {
    pub tool: String,
    pub resolution: String,
    pub project: String,
    /// Every config file found, with details and how many dirs it covers.
    pub configs: Vec<ConfigInstance>,
    /// Top-level uncovered directories (collapsed — children implied uncovered).
    pub uncovered: Vec<String>,
    pub summary: Summary,
}

#[derive(Serialize)]
pub struct ConfigInstance {
    pub path: String,
    pub details: serde_json::Value,
    pub covers_dirs: u32,
}

#[derive(Serialize)]
pub struct Summary {
    pub total_dirs: u32,
    pub covered_dirs: u32,
    pub uncovered_dirs: u32,
}

// ---------------------------------------------------------------------------
// Build coverage map
// ---------------------------------------------------------------------------

/// Build a coverage map for any tool.
///
/// 1. Gets all source directories from crawler
/// 2. For each, resolves which config covers it (walk-up or fixed)
/// 3. Collapses uncovered directories to top-level ancestors
pub fn build(tool: &dyn CoverageTool, root: &Path, crawl: &CrawlResult) -> CoverageMap {
    let config_files = tool.config_files(crawl);
    let source_dirs = tool.source_dirs(crawl);

    let mut covered_count: u32 = 0;
    let mut uncovered_dirs: Vec<PathBuf> = Vec::new();
    let mut config_cover_counts: std::collections::BTreeMap<String, u32> =
        std::collections::BTreeMap::new();

    for dir in source_dirs {
        let resolved = if tool.walks_up() {
            walk_up_resolve(dir, root, config_files)
        } else if config_files
            .iter()
            .any(|cf| cf.parent() == Some(dir.as_path()))
        {
            Some(dir.clone())
        } else {
            None
        };

        if let Some(config_dir) = resolved {
            covered_count = covered_count.saturating_add(1);
            if let Some(cf) = config_files
                .iter()
                .find(|cf| cf.parent() == Some(config_dir.as_path()))
            {
                let count = config_cover_counts.entry(rel_str(root, cf)).or_insert(0);
                *count = count.saturating_add(1);
            }
        } else {
            uncovered_dirs.push(dir.clone());
        }
    }

    let collapsed_uncovered = collapse_to_ancestors(&uncovered_dirs);

    let mut configs = Vec::new();
    for cf in config_files {
        let cf_rel = rel_str(root, cf);
        let details = tool.parse_details(cf);
        let covers = config_cover_counts.get(&cf_rel).copied().unwrap_or(0);
        configs.push(ConfigInstance {
            path: cf_rel,
            details,
            covers_dirs: covers,
        });
    }

    let total = u32::try_from(source_dirs.len()).unwrap_or(0);

    CoverageMap {
        tool: tool.name().to_owned(),
        resolution: tool.resolution_description().to_owned(),
        project: root
            .canonicalize()
            .unwrap_or_else(|_| root.to_path_buf())
            .display()
            .to_string(),
        configs,
        uncovered: collapsed_uncovered
            .iter()
            .map(|d| rel_str(root, d))
            .collect(),
        summary: Summary {
            total_dirs: total,
            covered_dirs: covered_count,
            uncovered_dirs: total.saturating_sub(covered_count),
        },
    }
}

/// Collapse uncovered directories to their highest common ancestors.
///
/// Two collapses:
/// 1. If a parent is in the list, don't list its children (direct ancestor collapse)
/// 2. If ALL siblings under a parent are uncovered, collapse to the parent
///    (even if the parent itself has no source files)
fn collapse_to_ancestors(dirs: &[PathBuf]) -> Vec<PathBuf> {
    if dirs.is_empty() {
        return Vec::new();
    }

    // Repeatedly collapse: if a parent has 2+ children in the set, replace with parent.
    let mut current: BTreeSet<PathBuf> = dirs.iter().cloned().collect();
    loop {
        let mut changed = false;
        let mut new_set = current.clone();

        // Group by parent
        let groups = group_by_parent(&current);

        // Collapse parents with 2+ children
        for (parent, children) in &groups {
            if children.len() >= 2 {
                for child in children {
                    let _removed = new_set.remove(child);
                }
                let _inserted = new_set.insert(parent.clone());
                changed = true;
            }
        }

        current = new_set;
        if !changed {
            break;
        }
    }

    // Remove any dir whose ancestor is already in the set
    let mut result: Vec<PathBuf> = Vec::new();
    for dir in &current {
        let has_ancestor = result.iter().any(|ancestor| dir.starts_with(ancestor));
        if !has_ancestor {
            result.push(dir.clone());
        }
    }

    result
}

type DirGroups = std::collections::BTreeMap<PathBuf, Vec<PathBuf>>;

/// Group paths by their parent directory.
fn group_by_parent(dirs: &BTreeSet<PathBuf>) -> DirGroups {
    let mut groups = DirGroups::new();
    for dir in dirs {
        if let Some(parent) = dir.parent() {
            groups
                .entry(parent.to_path_buf())
                .or_default()
                .push(dir.clone());
        }
    }
    groups
}

// ---------------------------------------------------------------------------
// Walk-up resolution
// ---------------------------------------------------------------------------

/// Simulate walk-up: from `start_dir`, walk up checking for config files.
/// Returns the DIRECTORY where the nearest config was found.
fn walk_up_resolve(
    start_dir: &Path,
    project_root: &Path,
    config_files: &[PathBuf],
) -> Option<PathBuf> {
    let mut current = start_dir;
    loop {
        if config_files.iter().any(|p| p.parent() == Some(current)) {
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
// Rendering
// ---------------------------------------------------------------------------

/// Print as JSON.
#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_json(map: &CoverageMap) {
    if let Ok(json) = serde_json::to_string_pretty(map) {
        println!("{json}");
    }
}

/// Print as human-readable tree.
#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_tree(map: &CoverageMap) {
    println!("{} coverage", map.tool);
    println!("({})\n", map.resolution);

    println!("Configs found:");
    for cfg in &map.configs {
        println!(
            "  {:<50} {} (covers {} dirs)",
            cfg.path,
            format_details(&cfg.details),
            cfg.covers_dirs
        );
    }

    if map.uncovered.is_empty() {
        println!("\nAll directories covered.");
    } else {
        println!("\n⚠ Uncovered directories:");
        for dir in &map.uncovered {
            println!("  {dir}/");
        }
    }

    println!(
        "\nSummary: {}/{} dirs covered, {} uncovered",
        map.summary.covered_dirs, map.summary.total_dirs, map.summary.uncovered_dirs
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn rel_str(root: &Path, path: &Path) -> String {
    let s = path
        .strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string();
    if s.is_empty() { ".".to_owned() } else { s }
}

fn format_details(details: &serde_json::Value) -> String {
    if let Some(obj) = details.as_object() {
        obj.iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        String::new()
    }
}
