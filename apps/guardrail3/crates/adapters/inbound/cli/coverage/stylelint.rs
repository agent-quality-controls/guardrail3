//! Stylelint coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (verified via source: stylelint 17.4.0, cosmiconfig 9.x, 2026-03-19):
//!
//! Walk-up from each linted CSS file (per-file, not per-CWD).
//! Cosmiconfig `searchStrategy: 'global'` — walks up to `$HOME`.
//! Nearest `.stylelintrc.*` or `stylelint.config.*` wins.
//! `extends` deep-merges rules (later entries override earlier).
//! Intermediate configs shadow for files below them.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct StylelintCoverage;

impl CoverageTool for StylelintCoverage {
    fn name(&self) -> &'static str {
        "stylelint"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from each CSS file (cosmiconfig global) — nearest .stylelintrc.* wins, extends deep-merges"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.stylelint_configs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_css
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
    let map = engine::build(&StylelintCoverage, root, crawl);
    engine::print(&map);
}
