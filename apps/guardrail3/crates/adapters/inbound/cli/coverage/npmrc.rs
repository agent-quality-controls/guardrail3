//! `.npmrc` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (verified via npm/pnpm source, 2026-03-19):
//!
//! npm walks up from CWD to find project root (first dir with `package.json`
//! or `node_modules`), then loads `.npmrc` from that project root.
//! pnpm loads root `.npmrc` for all workspace packages during `pnpm install`.
//! Settings cascade: project `.npmrc` > user `~/.npmrc` > global.
//! Higher-priority sources shadow lower ones (no merging within a level).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

#[derive(Debug)]
pub struct NpmrcCoverage;

impl CoverageTool for NpmrcCoverage {
    fn name(&self) -> &'static str {
        "npmrc"
    }

    fn resolution_description(&self) -> &'static str {
        "project root .npmrc (walk-up to package.json) — project > user > global cascade"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.npmrcs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let settings = guardrail3_shared_fs::read_file(config_path).map_or(0, |c| {
            c.lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.is_empty() && !t.starts_with('#')
                })
                .count()
        });
        serde_json::json!({"settings": settings})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&NpmrcCoverage, root, crawl);
    engine::print(&map);
}
