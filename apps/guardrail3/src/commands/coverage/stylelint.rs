//! Stylelint coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: cosmiconfig walk-up from each linted CSS file.
//!
//! Nearest `.stylelintrc.*` or `stylelint.config.*` wins.
//! `extends` MERGES rules (unlike most tools where nearest replaces).
//! But the config file itself must be found via walk-up first.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct StylelintCoverage;

impl CoverageTool for StylelintCoverage {
    fn name(&self) -> &'static str {
        "stylelint"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from each CSS file (cosmiconfig) — nearest .stylelintrc.* wins"
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
pub fn print_json(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&StylelintCoverage, root, crawl);
    engine::print_json(&map);
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print_tree(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&StylelintCoverage, root, crawl);
    engine::print_tree(&map);
}
