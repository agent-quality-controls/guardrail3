//! `clippy.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution: walk-up from `CARGO_MANIFEST_DIR`
//!
//! Checks `clippy.toml` and `.clippy.toml` at each parent directory.
//! Nearest wins, shadows completely. No merging.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app::crawl::CrawlResult;
use crate::app::rs::validate::clippy_coverage::{EXPECTED_METHOD_BANS, EXPECTED_TYPE_BANS};

use super::engine::{self, CoverageTool};

pub struct ClippyCoverage;

impl CoverageTool for ClippyCoverage {
    fn name(&self) -> &'static str {
        "clippy"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up from CARGO_MANIFEST_DIR — nearest clippy.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.clippy_tomls
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

        let methods = diff_bans(&table, "disallowed-methods", EXPECTED_METHOD_BANS);
        let types = diff_bans(&table, "disallowed-types", EXPECTED_TYPE_BANS);
        let thresholds = check_thresholds(&table);

        serde_json::json!({
            "methods": methods,
            "types": types,
            "thresholds": thresholds
        })
    }

    fn walks_up(&self) -> bool {
        true
    }
}

/// Diff actual ban entries against required baseline.
fn diff_bans(table: &toml::Value, key: &str, required: &[&str]) -> serde_json::Value {
    let entries: Vec<String> = table
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    entry
                        .get("path")
                        .and_then(|p| p.as_str())
                        .map(str::to_owned)
                })
                .collect()
        })
        .unwrap_or_default();

    let total = entries.len();
    let required_present = required
        .iter()
        .filter(|r| entries.iter().any(|e| e == **r))
        .count();
    let required_missing = required.len().saturating_sub(required_present);
    let user_extra = entries
        .iter()
        .filter(|e| !required.contains(&e.as_str()))
        .count();

    serde_json::json!({
        "total": total,
        "required_total": required.len(),
        "required_present": required_present,
        "required_missing": required_missing,
        "user_extra": user_extra
    })
}

/// Check thresholds against guardrail3 defaults.
fn check_thresholds(table: &toml::Value) -> serde_json::Value {
    type ThresholdDef = (&'static str, u64);
    let defaults: &[ThresholdDef] = &[
        ("too-many-lines-threshold", 75),
        ("cognitive-complexity-threshold", 15),
        ("too-many-arguments-threshold", 7),
        ("type-complexity-threshold", 75),
        ("max-struct-bools", 3),
    ];

    let total = defaults.len();
    let mut required_present = 0usize;
    let mut required_missing = 0usize;
    let mut relaxed = 0usize;
    let mut user_extra = 0usize;

    for (key, default_val) in defaults {
        if let Some(val) = table.get(*key).and_then(toml::Value::as_integer) {
            required_present = required_present.saturating_add(1);
            let val_u64 = u64::try_from(val).unwrap_or(0);
            if val_u64 > *default_val {
                relaxed = relaxed.saturating_add(1);
            }
        } else {
            required_missing = required_missing.saturating_add(1);
        }
    }

    // Check for threshold keys in the file that aren't in our baseline
    let known_keys: Vec<&str> = defaults.iter().map(|(k, _)| *k).collect();
    for (key, val) in table.as_table().iter().flat_map(|t| t.iter()) {
        if val.is_integer() && !known_keys.contains(&key.as_str()) {
            user_extra = user_extra.saturating_add(1);
        }
    }

    let total_in_file = required_present.saturating_add(user_extra);

    serde_json::json!({
        "total": total_in_file,
        "required_total": total,
        "required_present": required_present,
        "required_missing": required_missing,
        "user_extra": user_extra,
        "relaxed": relaxed
    })
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&ClippyCoverage, root, crawl);
    engine::print(&map);
}
