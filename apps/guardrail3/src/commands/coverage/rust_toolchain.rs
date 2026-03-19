//! `rust-toolchain.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from CWD
//!
//! Rustup walks up parent directories looking for `rust-toolchain.toml`
//! (or legacy `rust-toolchain`). Nearest wins.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct RustToolchainCoverage;

impl CoverageTool for RustToolchainCoverage {
    fn name(&self) -> &'static str {
        "rust-toolchain"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CWD — nearest rust-toolchain.toml wins"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.rust_toolchains
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_rs
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        let channel = content
            .lines()
            .find(|l| l.trim().starts_with("channel"))
            .and_then(|l| l.split('=').nth(1))
            .map_or("unknown", |s| s.trim().trim_matches('"'));
        serde_json::json!({"channel": channel})
    }

    fn walks_up(&self) -> bool {
        true
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&RustToolchainCoverage, root, crawl);
    engine::print(&map);
}
