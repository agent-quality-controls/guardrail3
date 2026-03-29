//! Prettier coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified with Prettier 3.8.1, 2026-03-19):
//!
//! Walk-up from each formatted file. Nearest `.prettierrc.*` or
//! `prettier.config.*` wins. Intermediate configs shadow completely.
//! NO extends/merge mechanism — subdirectory config replaces root entirely.
//! Uses editorconfig integration but that doesn't affect config discovery.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

#[derive(Debug)]
pub struct PrettierCoverage;

impl CoverageTool for PrettierCoverage {
    fn name(&self) -> &'static str {
        "prettier"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from each file — nearest .prettierrc.* wins, no merge/extends"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.prettier_configs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        // Prettier formats TS/JS and CSS
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let lines = guardrail3_shared_fs::read_file(config_path).map_or(0, |c| c.lines().count());
        serde_json::json!({"lines": lines})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&PrettierCoverage, root, crawl);
    engine::print(&map);
}
