//! `clippy.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from `CARGO_MANIFEST_DIR`
//!
//! Checks `clippy.toml` and `.clippy.toml` at each parent directory.
//! Nearest wins, shadows completely. No merging.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

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

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_rs
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
