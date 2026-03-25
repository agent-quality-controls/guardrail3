//! `rustfmt.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified 2026-03-19):
//!
//! Walk-up from source files. Nearest `rustfmt.toml` or `.rustfmt.toml` wins.
//! Shadows completely. Crosses workspace boundaries.
//! `.rustfmt.toml` (dot) takes priority over `rustfmt.toml` (no dot).
//! `cargo fmt -p <crate>` resolves per-crate — intermediate files shadow.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

/// Required rustfmt settings with their expected values.
type SettingDef = (&'static str, &'static str);
const REQUIRED_SETTINGS: &[SettingDef] = &[
    ("edition", "\"2024\""),
    ("max_width", "100"),
    ("tab_spaces", "4"),
    ("use_field_init_shorthand", "true"),
    ("use_try_shorthand", "true"),
    ("reorder_imports", "true"),
    ("reorder_modules", "true"),
];

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
        let Some(content) = guardrail3_shared_fs::read_file(config_path) else {
            return serde_json::json!({"error": "unreadable"});
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            return serde_json::json!({"error": "parse error"});
        };

        let settings = diff_settings(&table);
        serde_json::json!({
            "settings": settings
        })
    }

    fn walks_up(&self) -> bool {
        true
    }
}

fn diff_settings(table: &toml::Value) -> serde_json::Value {
    let mut required_present = 0usize;
    let mut required_missing = 0usize;
    let mut relaxed = 0usize;
    let mut user_extra = 0usize;

    for (key, expected) in REQUIRED_SETTINGS {
        if let Some(val) = table.get(*key) {
            required_present = required_present.saturating_add(1);
            let val_str = val.to_string();
            if val_str.trim() != *expected {
                relaxed = relaxed.saturating_add(1);
            }
        } else {
            required_missing = required_missing.saturating_add(1);
        }
    }

    // Count settings in the file that aren't in our required list
    let known_keys: Vec<&str> = REQUIRED_SETTINGS.iter().map(|(k, _)| *k).collect();
    if let Some(tbl) = table.as_table() {
        for key in tbl.keys() {
            if !known_keys.contains(&key.as_str()) {
                user_extra = user_extra.saturating_add(1);
            }
        }
    }

    let total = required_present.saturating_add(user_extra);

    serde_json::json!({
        "total": total,
        "required_total": REQUIRED_SETTINGS.len(),
        "required_present": required_present,
        "required_missing": required_missing,
        "user_extra": user_extra,
        "relaxed": relaxed
    })
}

#[allow(clippy::print_stdout)] // reason: CLI command
pub fn print(root: &Path, crawl: &CrawlResult) {
    let map = engine::build(&RustfmtCoverage, root, crawl);
    engine::print(&map);
}
