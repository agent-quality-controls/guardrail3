//! `rust-toolchain.toml` coverage — plugs into the generic coverage engine.
//!
//! ## Resolution (empirically verified 2026-03-19):
//!
//! Rustup walks up from CWD looking for `rust-toolchain.toml`.
//! Nearest wins, shadows completely. Crosses workspace boundaries.
//! Identical walk-up behavior to clippy/deny/rustfmt.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_app_core::crawl::CrawlResult;

use super::engine::{self, CoverageTool};

/// Required settings in `[toolchain]` section.
type SettingDef = (&'static str, &'static str);
const REQUIRED_SETTINGS: &[SettingDef] = &[("channel", "\"stable\"")];

/// Required components.
const REQUIRED_COMPONENTS: &[&str] = &["clippy", "rustfmt"];

pub struct RustToolchainCoverage;

impl CoverageTool for RustToolchainCoverage {
    fn name(&self) -> &'static str {
        "rust-toolchain"
    }

    fn resolution_description(&self) -> &'static str {
        "walk-up — nearest rust-toolchain.toml wins, shadows completely"
    }

    fn config_files<'a>(&self, crawl: &'a CrawlResult) -> &'a [PathBuf] {
        &crawl.rust_toolchains
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

        let toolchain = table.get("toolchain");

        // Check channel
        let mut required_present = 0usize;
        let mut required_missing = 0usize;
        let mut relaxed = 0usize;

        for (key, expected) in REQUIRED_SETTINGS {
            if let Some(val) = toolchain.and_then(|t| t.get(*key)) {
                required_present = required_present.saturating_add(1);
                if val.to_string().trim() != *expected {
                    relaxed = relaxed.saturating_add(1);
                }
            } else {
                required_missing = required_missing.saturating_add(1);
            }
        }

        // Check components
        let components: Vec<String> = toolchain
            .and_then(|t| t.get("components"))
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();

        let components_present = REQUIRED_COMPONENTS
            .iter()
            .filter(|r| components.iter().any(|c| c == **r))
            .count();
        let components_missing = REQUIRED_COMPONENTS.len().saturating_sub(components_present);

        let channel = toolchain
            .and_then(|t| t.get("channel"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        serde_json::json!({
            "channel": channel,
            "settings": {
                "required_total": REQUIRED_SETTINGS.len(),
                "required_present": required_present,
                "required_missing": required_missing,
                "relaxed": relaxed
            },
            "components": {
                "required_total": REQUIRED_COMPONENTS.len(),
                "required_present": components_present,
                "required_missing": components_missing
            }
        })
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
