//! cspell coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (verified via cspell.org docs, 2026-03-19):
//!
//! Walk-up from CWD (and from each file being checked). Searches parent
//! directories for cspell config files. Nearest config wins.
//! No auto-merge — subdirectory config must explicitly `import` parent.
//! `--stop-config-search-at` limits walk-up boundary.
//! `--no-config-search` disables walk-up entirely.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct CspellCoverage;

impl CoverageTool for CspellCoverage {
    fn name(&self) -> &'static str {
        "cspell"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CWD/file — nearest cspell config wins, import for explicit inheritance"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.cspell_configs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        // cspell checks both TS and Rust files
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        // Try to parse as JSON to get word count
        #[allow(clippy::disallowed_methods)] // reason: parsing config file, not untrusted input
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let words = json
                .get("words")
                .and_then(|w| w.as_array())
                .map_or(0, Vec::len);
            serde_json::json!({"words": words})
        } else {
            let lines = content.lines().count();
            serde_json::json!({"lines": lines})
        }
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&CspellCoverage, root, crawl);
    engine::print(&map);
}
