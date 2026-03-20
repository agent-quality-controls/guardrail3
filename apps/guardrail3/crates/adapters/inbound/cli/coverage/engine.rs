//! Generic coverage map engine.
//!
//! Shared walk-up resolution, data model, and rendering for all config file types.
//! Each tool module implements `CoverageTool` and plugs into this engine.
//!
//! Coverage is directory-based. Every directory with source files is a target.
//! Both covered and uncovered directories are collapsed to top-level ancestors.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::app::core::crawl::CrawlResult;

// ---------------------------------------------------------------------------
// Tool trait
// ---------------------------------------------------------------------------

/// What a tool-specific coverage module provides.
pub trait CoverageTool {
    /// Tool name (e.g., "clippy", "deny", "eslint").
    fn name(&self) -> &'static str;

    /// How the tool finds its config.
    fn resolution_description(&self) -> &'static str;

    /// All instances of this config file from the crawl result.
    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf];

    /// Directories containing source files this tool checks.
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
    pub configs: Vec<ConfigInstance>,
    pub uncovered: Vec<String>,
    pub summary: Summary,
}

#[derive(Serialize)]
pub struct ConfigInstance {
    pub path: String,
    pub details: serde_json::Value,
    /// Top-level directories covered by this config (collapsed).
    pub covers: Vec<String>,
    /// If this config is a shadow — it intercepts coverage from a parent config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shadow: Option<bool>,
    /// Which parent config this shadows (if `is_shadow` is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadows: Option<String>,
    /// Configs below this one that steal part of its coverage.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shadowed_by: Vec<ShadowedBy>,
}

#[derive(Serialize)]
pub struct ShadowedBy {
    pub path: String,
    pub steals: Vec<String>,
}

#[derive(Serialize)]
pub struct Summary {
    pub total_dirs: u32,
    pub covered_dirs: u32,
    pub uncovered_dirs: u32,
}

// ---------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------

/// Build a coverage map for any tool.
pub fn build(tool: &dyn CoverageTool, root: &Path, crawl: &CrawlResult) -> CoverageMap {
    let config_files = tool.config_files(crawl);
    let source_dirs = tool.source_dirs(crawl);

    // For each source dir, resolve which config covers it
    let mut covered_count: u32 = 0;
    let mut uncovered_dirs: Vec<PathBuf> = Vec::new();
    let mut config_covered_dirs: CoveredDirsMap = CoveredDirsMap::new();

    for dir in source_dirs {
        let resolved = if tool.walks_up() {
            walk_up_resolve(dir, root, config_files)
        } else {
            // Non-walk-up tools (e.g., jscpd): a config at directory X covers
            // all source dirs under X, not just at X. Find the nearest ancestor
            // config directory.
            config_files
                .iter()
                .filter_map(|cf| cf.parent())
                .filter(|config_dir| dir.starts_with(config_dir))
                .max_by_key(|config_dir| config_dir.components().count())
                .map(Path::to_path_buf)
        };

        if let Some(config_dir) = resolved {
            covered_count = covered_count.saturating_add(1);
            if let Some(cf) = config_files
                .iter()
                .find(|cf| cf.parent() == Some(config_dir.as_path()))
            {
                config_covered_dirs
                    .entry(rel_str(root, cf))
                    .or_default()
                    .push(dir.clone());
            }
        } else {
            uncovered_dirs.push(dir.clone());
        }
    }

    // Build config instances with collapsed covers
    let mut configs: Vec<ConfigInstance> = Vec::new();
    for cf in config_files {
        let cf_rel = rel_str(root, cf);
        let details = tool.parse_details(cf);
        let count = config_covered_dirs.get(&cf_rel).map_or(0, Vec::len);
        let config_dir = cf.parent().map(|d| rel_str(root, d)).unwrap_or_default();
        configs.push(ConfigInstance {
            path: cf_rel,
            details,
            covers: if count > 0 { vec![config_dir] } else { vec![] },
            is_shadow: None,
            shadows: None,
            shadowed_by: Vec::new(),
        });
    }

    // Detect shadows: if config A's directory is below config B's directory,
    // A is a shadow of B (A steals coverage from B).
    if tool.walks_up() {
        detect_shadows(&mut configs);
    }

    let collapsed_uncovered = collapse_to_ancestors(&uncovered_dirs, root);
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

/// Detect shadow relationships between configs.
///
/// Config A shadows config B if A's coverage directory is nested inside B's.
/// A intercepts walk-up resolution for source dirs under A, stealing them from B.
type IndexedDir = (usize, String);
type ShadowRelation = (usize, usize, String);
type ShadowMark = (usize, String);
type ParentMark = (usize, String, String);

fn detect_shadows(configs: &mut [ConfigInstance]) {
    let config_dirs: Vec<IndexedDir> = configs
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.covers.first().map(|d| (i, d.clone())))
        .collect();

    let mut shadows: Vec<ShadowRelation> = Vec::new();
    for &(i, ref dir_i) in &config_dirs {
        for &(j, ref dir_j) in &config_dirs {
            if i != j && dir_i != dir_j && Path::new(dir_i).starts_with(Path::new(dir_j)) {
                shadows.push((i, j, dir_i.clone()));
            }
        }
    }

    let mut shadow_marks: Vec<ShadowMark> = Vec::new();
    let mut parent_marks: Vec<ParentMark> = Vec::new();

    for (shadow_idx, parent_idx, stolen_dir) in &shadows {
        if let Some(parent) = configs.get(*parent_idx) {
            shadow_marks.push((*shadow_idx, parent.path.clone()));
        }
        if let Some(shadow) = configs.get(*shadow_idx) {
            parent_marks.push((*parent_idx, shadow.path.clone(), stolen_dir.clone()));
        }
    }

    for (idx, parent_path) in shadow_marks {
        if let Some(cfg) = configs.get_mut(idx) {
            cfg.is_shadow = Some(true);
            cfg.shadows = Some(parent_path);
        }
    }

    for (idx, shadow_path, stolen_dir) in parent_marks {
        if let Some(cfg) = configs.get_mut(idx) {
            cfg.shadowed_by.push(ShadowedBy {
                path: shadow_path,
                steals: vec![stolen_dir],
            });
        }
    }
}

/// Collapse directories to their highest common ancestors.
///
/// If a parent has 2+ children in the set, replace them with the parent.
/// Repeat until stable. Then remove any dir whose ancestor is already present.
/// Never collapse to an empty path (project root `.`) — stop one level above.
fn collapse_to_ancestors(dirs: &[PathBuf], project_root: &Path) -> Vec<PathBuf> {
    if dirs.is_empty() {
        return Vec::new();
    }

    let mut current: BTreeSet<PathBuf> = dirs.iter().cloned().collect();
    loop {
        let mut changed = false;
        let mut new_set = current.clone();
        let groups = group_by_parent(&current);

        for (parent, children) in &groups {
            // Don't collapse beyond one level below project root
            // Keep top-level dirs (apps/, packages/, tools/) visible
            if parent == project_root || parent.parent() == Some(project_root) {
                continue;
            }
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

    let mut result: Vec<PathBuf> = Vec::new();
    for dir in &current {
        let has_ancestor = result.iter().any(|a| dir.starts_with(a));
        if !has_ancestor {
            result.push(dir.clone());
        }
    }
    result
}

type DirGroups = BTreeMap<PathBuf, Vec<PathBuf>>;
type CoveredDirsMap = BTreeMap<String, Vec<PathBuf>>;

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

/// Print coverage map as JSON (the only output format).
#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(map: &CoverageMap) {
    if let Ok(json) = serde_json::to_string_pretty(map) {
        println!("{json}");
    }
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
