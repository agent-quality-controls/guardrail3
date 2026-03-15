use std::path::Path;
use std::process::Command;

use crate::report::types::{CheckResult, Severity};

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

pub fn check(workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R45: cargo-deny installed
    check_tool_installed("cargo-deny", "R45", Severity::Error, &mut results);

    // R46: cargo-machete installed
    check_tool_installed("cargo-machete", "R46", Severity::Error, &mut results);

    // R47: cargo-dupes installed (recommended, not required)
    check_tool_installed("cargo-dupes", "R47", Severity::Warn, &mut results);

    // R48: gitleaks installed
    check_tool_installed("gitleaks", "R48", Severity::Error, &mut results);

    // R50: Banned crates in Cargo.lock
    check_cargo_lock(workspace_root, &mut results);

    results
}

fn check_tool_installed(
    tool: &str,
    check_id: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    let cmd_result = Command::new("which")
        .arg(tool)
        .output();

    match cmd_result {
        Ok(output) if output.status.success() => {
            results.push(CheckResult {
                id: check_id.to_string(),
                severity: Severity::Info,
                title: format!("{tool} installed"),
                message: format!("{tool} found on PATH"),
                file: None,
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: check_id.to_string(),
                severity: missing_severity,
                title: format!("{tool} not installed"),
                message: format!("{tool} not found — install with: cargo install {tool}"),
                file: None,
                line: None,
            });
        }
    }
}

fn check_cargo_lock(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let lock_path = workspace_root.join("Cargo.lock");
    if !lock_path.exists() {
        results.push(CheckResult {
            id: "R50".to_string(),
            severity: Severity::Warn,
            title: "Cargo.lock not found".to_string(),
            message: "Cannot check for banned crates without Cargo.lock".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
        return;
    }

    let content = match std::fs::read_to_string(&lock_path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R50".to_string(),
                severity: Severity::Error,
                title: "Cargo.lock unreadable".to_string(),
                message: format!("Failed to read: {e}"),
                file: Some(lock_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R50".to_string(),
                severity: Severity::Error,
                title: "Cargo.lock parse error".to_string(),
                message: format!("Invalid TOML: {e}"),
                file: Some(lock_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let packages = match table.get("package").and_then(|p| p.as_array()) {
        Some(arr) => arr,
        None => return,
    };

    let mut found_banned = Vec::new();
    for pkg in packages {
        if let Some(name) = pkg.get("name").and_then(|n| n.as_str()) {
            if BANNED_CRATE_NAMES.contains(&name) {
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
            id: "R50".to_string(),
            severity: Severity::Info,
            title: "No banned crates in lockfile".to_string(),
            message: "Cargo.lock is clean".to_string(),
            file: Some(lock_path.display().to_string()),
            line: None,
        });
    } else {
        for banned in &found_banned {
            results.push(CheckResult {
                id: "R50".to_string(),
                severity: Severity::Error,
                title: "Banned crate in lockfile".to_string(),
                message: format!("Found banned crate: {banned}"),
                file: Some(lock_path.display().to_string()),
                line: None,
            });
        }
    }
}
