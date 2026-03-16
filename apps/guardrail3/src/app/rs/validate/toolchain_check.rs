use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

pub fn check_toolchain_settings(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let content = match fs.read_file_err(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(path.display().to_string()),
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
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    check_toolchain_channel(&table, path, results);
    check_toolchain_components(&table, path, results);
}

fn check_toolchain_channel(
    table: &toml::Value,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
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
                inventory: false,
            }.as_inventory());
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "Toolchain channel not stable".to_owned(),
                message: format!("channel = \"{other}\" but should be \"stable\". Set `channel = \"stable\"` in [toolchain] in rust-toolchain.toml."),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "Toolchain channel missing".to_owned(),
                message: "Toolchain channel not set. Add `channel = \"stable\"` to [toolchain] in rust-toolchain.toml.".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_toolchain_components(
    table: &toml::Value,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
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
                        inventory: false,
                    });
                } else {
                    results.push(CheckResult {
                        id: "R25".to_owned(),
                        severity: Severity::Warn,
                        title: format!("Component {expected} missing"),
                        message: format!("`{expected}` missing from components. Add `\"{expected}\"` to `components` array in [toolchain] in rust-toolchain.toml."),
                        file: Some(path.display().to_string()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
        None => {
            results.push(CheckResult {
                id: "R25".to_owned(),
                severity: Severity::Warn,
                title: "No components list".to_owned(),
                message: "No components list in [toolchain]. Add `components = [\"clippy\", \"rustfmt\"]` to rust-toolchain.toml.".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
