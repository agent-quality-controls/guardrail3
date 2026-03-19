//! `deny.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from manifest directory
//!
//! Checks `deny.toml`, `.deny.toml`, `.cargo/deny.toml` at each parent directory.
//! Nearest wins, shadows completely.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct DenyCoverage;

impl CoverageTool for DenyCoverage {
    fn name(&self) -> &'static str {
        "deny"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from manifest directory — nearest deny.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.deny_tomls
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
        serde_json::json!({"bans": bans, "advisory_ignores": advisory_ignores})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_json(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&DenyCoverage, root, crawl);
    engine::print_json(&map);
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_tree(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&DenyCoverage, root, crawl);
    engine::print_tree(&map);
}
