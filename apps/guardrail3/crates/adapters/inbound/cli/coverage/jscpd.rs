//! `.jscpd.json` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified + source verified, 2026-03-19):
//!
//! CWD only — NO walk-up. jscpd does NOT use cosmiconfig at all.
//! Config resolution: `path.resolve(".jscpd.json")` (CWD + filename).
//! Also checks `package.json` `"jscpd"` key in CWD.
//! Parent directory configs are completely ignored.
//! `--config <path>` flag overrides to explicit path.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct JscpdCoverage;

impl CoverageTool for JscpdCoverage {
    fn name(&self) -> &'static str {
        "jscpd"
    }

    fn resolution_description(&self) -> &'static str {
        "CWD only (no walk-up, no cosmiconfig) — .jscpd.json resolved from CWD"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.jscpd_configs
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        // jscpd checks both TS and Rust
        &crawl.dirs_with_ts
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({});
        };
        #[allow(clippy::disallowed_methods)] // reason: parsing config file
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let threshold = json
                .get("threshold")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            serde_json::json!({"threshold": threshold})
        } else {
            serde_json::json!({})
        }
    }

    fn walks_up(&self) -> bool {
        false
    }
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&JscpdCoverage, root, crawl);
    engine::print(&map);
}
