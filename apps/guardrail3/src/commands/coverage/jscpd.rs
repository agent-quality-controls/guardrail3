//! `.jscpd.json` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: CWD only (no walk-up)
//!
//! cosmiconfig v9 default `searchStrategy: 'none'` disables parent search.
//! Config only found at CWD. `--config` flag overrides.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

pub struct JscpdCoverage;

impl CoverageTool for JscpdCoverage {
    fn name(&self) -> &'static str {
        "jscpd"
    }

    fn resolution_description(&self) -> &'static str {
        "CWD only (cosmiconfig v9, no walk-up)"
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
