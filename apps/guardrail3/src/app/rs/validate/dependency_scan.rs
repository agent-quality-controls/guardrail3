use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::{FileSystem, ToolChecker};

const BANNED_CRATE_NAMES: &[&str] = &[
    "simd-json",
    "json5",
    "sonic-rs",
    "openssl",
    "openssl-sys",
    "ureq",
    "surf",
    "isahc",
    "log4rs",
    "env_logger",
    "simple_logger",
    "fern",
    "async-std",
    "smol",
    "anyhow",
    "actix-web",
    "rocket",
    "warp",
    "poem",
    "chrono",
    "diesel",
    "sea-orm",
    "bincode",
    "rmp-serde",
    "prost",
    "flatbuffers",
];

pub fn check(fs: &dyn FileSystem, tc: &dyn ToolChecker, workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R45: cargo-deny installed
    check_tool_installed(tc, "cargo-deny", "R45", Severity::Error, &mut results);

    // R46: cargo-machete installed
    check_tool_installed(tc, "cargo-machete", "R46", Severity::Error, &mut results);

    // R47: cargo-dupes installed (recommended, not required)
    check_tool_installed(tc, "cargo-dupes", "R47", Severity::Warn, &mut results);

    // R48: gitleaks installed
    check_tool_installed(tc, "gitleaks", "R48", Severity::Error, &mut results);

    // R50: Banned crates in Cargo.lock
    check_cargo_lock(fs, workspace_root, &mut results);

    results
}

fn check_tool_installed(
    tc: &dyn ToolChecker,
    tool: &str,
    check_id: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    if tc.is_installed(tool) {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Info,
            title: format!("{tool} installed"),
            message: format!("{tool} found on PATH"),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: missing_severity,
            title: format!("{tool} not installed"),
            message: format!("{tool} not found — install with: cargo install {tool}"),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

/// Parse deny.toml skip entries to get crate names that should be excluded from R50.
fn parse_deny_skip_crate_names(fs: &dyn FileSystem, workspace_root: &Path) -> BTreeSet<String> {
    let mut skipped = BTreeSet::new();
    let deny_path = workspace_root.join("deny.toml");
    let Some(content) = fs.read_file(&deny_path) else {
        return skipped;
    };
    let Ok(table) = content.parse::<toml::Value>() else {
        return skipped;
    };
    let Some(bans) = table.get("bans") else {
        return skipped;
    };
    let Some(skip) = bans.get("skip").and_then(|s| s.as_array()) else {
        return skipped;
    };

    for entry in skip {
        // Format: { crate = "name@version" }
        if let Some(crate_field) = entry.get("crate").and_then(|c| c.as_str()) {
            let name = crate_field.split('@').next().unwrap_or(crate_field);
            let _ = skipped.insert(name.to_owned());
        } else if let Some(s) = entry.as_str() {
            let _ = skipped.insert(s.to_owned());
        } else if let Some(n) = entry.get("name").and_then(|n| n.as_str()) {
            let _ = skipped.insert(n.to_owned());
        }
    }

    skipped
}

#[allow(clippy::too_many_lines)] // reason: cargo lock scanning
fn check_cargo_lock(fs: &dyn FileSystem, workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let lock_path = workspace_root.join("Cargo.lock");
    if !lock_path.exists() {
        results.push(CheckResult {
            id: "R50".to_owned(),
            severity: Severity::Warn,
            title: "Cargo.lock not found".to_owned(),
            message: "Cannot check for banned crates without Cargo.lock".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    let content = match fs.read_file_err(&lock_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R50".to_owned(),
                severity: Severity::Error,
                title: "Cargo.lock unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(lock_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R50".to_owned(),
                severity: Severity::Error,
                title: "Cargo.lock parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(lock_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let Some(packages) = table.get("package").and_then(|p| p.as_array()) else {
        return;
    };

    let skipped = parse_deny_skip_crate_names(fs, workspace_root);

    let mut found_banned = Vec::new();
    for pkg in packages {
        if let Some(name) = pkg.get("name").and_then(|n| n.as_str()) {
            if BANNED_CRATE_NAMES.contains(&name) && !skipped.contains(name) {
                let version = pkg
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                found_banned.push(format!("{name}@{version}"));
            }
        }
    }

    if found_banned.is_empty() {
        results.push(CheckResult {
            id: "R50".to_owned(),
            severity: Severity::Info,
            title: "No banned crates in lockfile".to_owned(),
            message: "Cargo.lock is clean".to_owned(),
            file: Some(lock_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        for banned in &found_banned {
            results.push(CheckResult {
                id: "R50".to_owned(),
                severity: Severity::Error,
                title: "Banned crate in lockfile".to_owned(),
                message: format!("Found banned crate: {banned}"),
                file: Some(lock_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
