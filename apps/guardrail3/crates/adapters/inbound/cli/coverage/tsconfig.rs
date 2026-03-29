//! `tsconfig.json` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified with TypeScript 5.9.3, 2026-03-19):
//!
//! Walk-up from CWD. Nearest `tsconfig.json` wins (exact filename only —
//! `tsconfig.base.json` is NOT auto-discovered). Intermediate configs shadow.
//! With `-p`, uses the specified config directly.
//!
//! Key difference from Rust tools: `extends` provides deep-merge inheritance
//! from a parent config. But the walk-up itself is still nearest-wins shadow.
//! Without `extends`, the found file is the entire config (no implicit merging).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

#[derive(Debug)]
pub struct TsconfigCoverage;

impl CoverageTool for TsconfigCoverage {
    fn name(&self) -> &'static str {
        "tsconfig"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CWD — nearest tsconfig.json wins, extends provides deep-merge inheritance"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.tsconfigs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = guardrail3_shared_fs::read_file(config_path) else {
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
