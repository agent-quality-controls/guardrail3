//! `rustfmt.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from source file
//!
//! `rustfmt` directly: walks up from each source file.
//! `cargo fmt`: starts from workspace root only (different behavior!).
//! Coverage map simulates walk-up (matches rustfmt direct + IDE behavior).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct RustfmtCoverage;

impl CoverageTool for RustfmtCoverage {
    fn name(&self) -> &'static str {
        "rustfmt"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from source file — nearest rustfmt.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.rustfmt_tomls
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_rs
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        let settings = content
            .lines()
            .filter(|l| {
                let trimmed = l.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .count();
        serde_json::json!({"settings": settings})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&RustfmtCoverage, root, crawl);
    engine::print(&map);
}
