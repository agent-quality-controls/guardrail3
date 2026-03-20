//! `ESLint` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified with `ESLint` v10.0.3, 2026-03-19):
//!
//! Walk-up from each linted file. Nearest `eslint.config.*` wins.
//! No cascade, no merging. Complete replacement — ALL root rules lost.
//! Crosses directory boundaries. Intermediate configs shadow.
//! Same walk-up + shadow behavior as all Rust tools.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct EslintCoverage;

impl CoverageTool for EslintCoverage {
    fn name(&self) -> &'static str {
        "eslint"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from each linted file (ESLint v10 default) — nearest eslint.config.* wins"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.eslint_configs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let lines = crate::fs::read_file(config_path).map_or(0, |c| c.lines().count());
        serde_json::json!({"lines": lines})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&EslintCoverage, root, crawl);
    engine::print(&map);
}
