//! `.npmrc` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: project root level for workspace installs
//!
//! pnpm reads root `.npmrc` for all workspace packages during `pnpm install`.
//! Per-package `.npmrc` only used for direct invocation in that directory.
//! Walk-up behavior: pnpm checks project, user, global levels.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct NpmrcCoverage;

impl CoverageTool for NpmrcCoverage {
    fn name(&self) -> &'static str {
        "npmrc"
    }

    fn resolution_description(&self) -> &'static str {
        "project root for workspace installs, per-directory for direct pnpm invocation"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.npmrcs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let settings = crate::fs::read_file(config_path).map_or(0, |c| {
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
