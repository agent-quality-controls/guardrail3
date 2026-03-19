//! `tsconfig.json` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from CWD (when no -p or files given)
//!
//! `tsc` walks up from CWD looking for `tsconfig.json`.
//! With `-p`, uses the specified config. No cascade.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct TsconfigCoverage;

impl CoverageTool for TsconfigCoverage {
    fn name(&self) -> &'static str {
        "tsconfig"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CWD — nearest tsconfig.json wins"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.tsconfigs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        let has_extends = content.contains("\"extends\"");
        let has_strict =
            content.contains("\"strict\": true") || content.contains("\"strict\":true");
        serde_json::json!({
            "extends": has_extends,
            "strict": has_strict
        })
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&TsconfigCoverage, root, crawl);
    engine::print(&map);
}
