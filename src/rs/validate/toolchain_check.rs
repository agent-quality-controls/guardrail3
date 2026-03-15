use std::path::Path;

use crate::report::types::{CheckResult, Severity};

#[allow(clippy::too_many_lines)] // reason: toolchain settings validation
pub fn check_toolchain_settings(path: &Path, results: &mut Vec<CheckResult>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // Check channel = "stable"
    let channel = table
        .get("toolchain")
        .and_then(|t| t.get("channel"))
        .and_then(|c| c.as_str());

    match channel {
        Some("stable") => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Info,
                title: "Toolchain channel correct".to_owned(),
                message: "channel = \"stable\"".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "Toolchain channel not stable".to_owned(),
                message: format!("channel = \"{other}\", expected \"stable\""),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "Toolchain channel missing".to_owned(),
                message: "Expected [toolchain] channel = \"stable\"".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
    }

    // Check components include clippy + rustfmt
    let components = table
        .get("toolchain")
        .and_then(|t| t.get("components"))
        .and_then(|c| c.as_array());

    match components {
        Some(arr) => {
            let comp_strs: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();

            for expected in &["clippy", "rustfmt"] {
                if comp_strs.contains(expected) {
                    results.push(CheckResult {
                        id: "R25".to_owned(),
                        severity: Severity::Info,
                        title: format!("Component {expected} present"),
                        message: format!("{expected} in components list"),
                        file: Some(path.display().to_string()),
                        line: None,
                    });
                } else {
                    results.push(CheckResult {
                        id: "R25".to_owned(),
                        severity: Severity::Warn,
                        title: format!("Component {expected} missing"),
                        message: format!("{expected} not in components list"),
                        file: Some(path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
        None => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "No components list".to_owned(),
                message: "Expected [toolchain] components = [\"clippy\", \"rustfmt\"]".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
    }
}
