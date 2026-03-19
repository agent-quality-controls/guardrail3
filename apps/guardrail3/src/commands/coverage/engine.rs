//! Generic coverage map engine.
//!
//! Shared walk-up resolution, data model, and rendering for all config file types.
//! Each tool module implements `CoverageTool` and plugs into this engine.

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

    /// Extract all targets that need coverage (crates, apps, packages).
    fn targets(&self, crawl: &CrawlResult, root: &Path) -> Vec<Target>;

    /// Parse tool-specific details from a config file (e.g., ban count, rule count).
    fn parse_details(&self, config_path: &Path) -> serde_json::Value;

    /// Whether this tool walks up parent directories (true for most, false for jscpd/gitleaks).
    fn walks_up(&self) -> bool;
}

/// A target that needs coverage — a crate, app, or package directory.
pub struct Target {
    pub name: String,
    pub path: PathBuf,
    pub structure_file: PathBuf,
    pub structure_kind: &'static str,
    /// If this target is a member of a group (workspace), the group's path.
    pub group: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// Output data model (serialized to JSON)
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct CoverageMap {
    pub tool: String,
    pub resolution: String,
    pub project: String,
    pub configs: Vec<ConfigInstance>,
    pub targets: Vec<TargetCoverage>,
    pub summary: Summary,
}

#[derive(Serialize)]
pub struct ConfigInstance {
    pub path: String,
    pub details: serde_json::Value,
    pub covers: Vec<String>,
}

#[derive(Serialize)]
pub struct TargetCoverage {
    pub kind: String,
    pub name: String,
    pub path: String,
    pub structure_file: String,
    pub covered_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Serialize)]
pub struct Summary {
    pub total_targets: u32,
    pub covered: u32,
    pub uncovered: u32,
}

// ---------------------------------------------------------------------------
// Build coverage map
// ---------------------------------------------------------------------------

/// Build a coverage map for any tool.
pub fn build(tool: &dyn CoverageTool, root: &Path, crawl: &CrawlResult) -> CoverageMap {
    let config_files = tool.config_files(crawl);
    let targets = tool.targets(crawl, root);

    // Resolve coverage for each target
    let mut target_coverages = Vec::new();
    for target in &targets {
        let covered_by = if tool.walks_up() {
            walk_up_resolve(&root.join(&target.path), root, config_files).map(|p| rel_str(root, &p))
        } else {
            // Non-walk-up: check if config exists at target's own directory
            if config_files
                .iter()
                .any(|cf| cf.parent() == Some(&root.join(&target.path)))
            {
                Some(rel_str(root, &root.join(&target.path)))
            } else {
                None
            }
        };

        // For walk-up configs: format as "dir/config_filename"
        let covered_by_display = covered_by.as_ref().map(|dir| {
            // Find the actual config file at that directory
            let dir_abs = root.join(dir);
            config_files
                .iter()
                .find(|cf| cf.parent() == Some(dir_abs.as_path()))
                .map_or_else(|| format!("{dir}/<config>"), |cf| rel_str(root, cf))
        });

        target_coverages.push(TargetCoverage {
            kind: target.structure_kind.to_owned(),
            name: target.name.clone(),
            path: target.path.display().to_string(),
            structure_file: target.structure_file.display().to_string(),
            covered_by: covered_by_display,
            group: target.group.as_ref().map(|g| g.display().to_string()),
        });
    }

    // Build config instances with coverage info
    let mut config_instances = Vec::new();
    for cf in config_files {
        let cf_rel = rel_str(root, cf);
        let details = tool.parse_details(cf);

        let covers: Vec<String> = target_coverages
            .iter()
            .filter(|t| t.covered_by.as_deref() == Some(cf_rel.as_str()))
            .map(|t| t.path.clone())
            .collect();

        config_instances.push(ConfigInstance {
            path: cf_rel,
            details,
            covers,
        });
    }

    // Summary
    let total = u32::try_from(target_coverages.len()).unwrap_or(0);
    let covered = u32::try_from(
        target_coverages
            .iter()
            .filter(|t| t.covered_by.is_some())
            .count(),
    )
    .unwrap_or(0);

    CoverageMap {
        tool: tool.name().to_owned(),
        resolution: tool.resolution_description().to_owned(),
        project: root
            .canonicalize()
            .unwrap_or_else(|_| root.to_path_buf())
            .display()
            .to_string(),
        configs: config_instances,
        targets: target_coverages,
        summary: Summary {
            total_targets: total,
            covered,
            uncovered: total.saturating_sub(covered),
        },
    }
}

// ---------------------------------------------------------------------------
// Walk-up resolution
// ---------------------------------------------------------------------------

/// Simulate walk-up config resolution: from `start_dir`, walk up parent
/// directories checking if any config file exists at each level.
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

    // Group targets by their group (workspace/scope) or show ungrouped
    let mut current_group: Option<&str> = None;

    for target in &map.targets {
        let group_str = target.group.as_deref();

        // Print group header when group changes
        if group_str != current_group {
            if let Some(g) = group_str {
                // Find config for this group
                let group_config = map.configs.iter().find(|c| {
                    c.path
                        .strip_suffix(&c.path.rsplit('/').next().unwrap_or(&c.path))
                        .unwrap_or("")
                        .trim_end_matches('/')
                        == g
                });
                println!("{g}/");
                if let Some(cfg) = group_config {
                    println!(
                        "  {} {}",
                        cfg.path.rsplit('/').next().unwrap_or(&cfg.path),
                        format_details(&cfg.details)
                    );
                }
            }
            current_group = group_str;
        }

        let indent = if target.group.is_some() { "    " } else { "" };
        let covered_str = match &target.covered_by {
            Some(c) => format!("covered by {c}"),
            None => "⚠ UNCOVERED".to_owned(),
        };

        println!(
            "{indent}{:<40} [{}] {} — {covered_str}",
            target.path, target.kind, target.name
        );
    }

    println!(
        "\nSummary: {}/{} targets covered, {} uncovered",
        map.summary.covered, map.summary.total_targets, map.summary.uncovered
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
