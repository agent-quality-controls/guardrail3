//! `deny.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from manifest directory
//!
//! Checks `deny.toml`, `.deny.toml`, `.cargo/deny.toml` at each parent directory.
//! Nearest wins, shadows completely.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::core::crawl::CrawlResult;
use crate::app::rs::validate::deny_audit::EXPECTED_BANS;

use super::engine::{self, CoverageTool};

pub struct DenyCoverage;

impl CoverageTool for DenyCoverage {
    fn name(&self) -> &'static str {
        "deny"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CWD (must have Cargo.toml) — nearest deny.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.deny_tomls
    }

    fn source_dirs<'a>(&self, crawl: &'a CrawlResult) -> &'a BTreeSet<PathBuf> {
        &crawl.dirs_with_rs
    }

    fn parse_details(&self, config_path: &Path) -> serde_json::Value {
        let Some(content) = crate::fs::read_file(config_path) else {
            return serde_json::json!({"error": "unreadable"});
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            return serde_json::json!({"error": "parse error"});
        };

        let bans = diff_bans(&table);

        serde_json::json!({
            "bans": bans
        })
    }

    fn walks_up(&self) -> bool {
        true
    }
}

/// Diff actual ban entries against required baseline.
fn diff_bans(table: &toml::Value) -> serde_json::Value {
    let entries: Vec<String> = table
        .get("bans")
        .and_then(|b| b.get("deny"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    entry
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map(str::to_owned)
                })
                .collect()
        })
        .unwrap_or_default();

    let total = entries.len();
    let required_present = EXPECTED_BANS
        .iter()
        .filter(|r| entries.iter().any(|e| e == **r))
        .count();
    let required_missing = EXPECTED_BANS.len().saturating_sub(required_present);
    let user_extra = entries
        .iter()
        .filter(|e| !EXPECTED_BANS.contains(&e.as_str()))
        .count();

    serde_json::json!({
        "total": total,
        "required_total": EXPECTED_BANS.len(),
        "required_present": required_present,
        "required_missing": required_missing,
        "user_extra": user_extra
    })
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&DenyCoverage, root, crawl);
    engine::print(&map);
}
