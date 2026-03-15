use std::collections::BTreeSet;
use std::path::Path;

use crate::report::types::{CheckResult, Severity};

pub fn check(workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R1: clippy.toml exists at workspace root
    let clippy_path = workspace_root.join("clippy.toml");
    if clippy_path.exists() {
        results.push(CheckResult {
            id: "R1".to_string(),
            severity: Severity::Info,
            title: "clippy.toml exists".to_string(),
            message: "Found at workspace root".to_string(),
            file: Some(clippy_path.display().to_string()),
            line: None,
        });

        // R3: Thresholds
        check_clippy_thresholds(&clippy_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R1".to_string(),
            severity: Severity::Error,
            title: "clippy.toml missing".to_string(),
            message: "No clippy.toml found at workspace root".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }

    // R21: rustfmt.toml exists
    let rustfmt_path = workspace_root.join("rustfmt.toml");
    if rustfmt_path.exists() {
        results.push(CheckResult {
            id: "R21".to_string(),
            severity: Severity::Info,
            title: "rustfmt.toml exists".to_string(),
            message: "Found at workspace root".to_string(),
            file: Some(rustfmt_path.display().to_string()),
            line: None,
        });

        // R22: rustfmt.toml settings differ (Warn)
        // R23: rustfmt.toml extra settings (Info)
        check_rustfmt_settings(&rustfmt_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R21".to_string(),
            severity: Severity::Error,
            title: "rustfmt.toml missing".to_string(),
            message: "No rustfmt.toml found at workspace root".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }

    // R24: rust-toolchain.toml exists — Error if missing
    let toolchain_path = workspace_root.join("rust-toolchain.toml");
    if toolchain_path.exists() {
        results.push(CheckResult {
            id: "R24".to_string(),
            severity: Severity::Info,
            title: "rust-toolchain.toml exists".to_string(),
            message: "Found at workspace root".to_string(),
            file: Some(toolchain_path.display().to_string()),
            line: None,
        });

        // R25: rust-toolchain.toml settings
        check_toolchain_settings(&toolchain_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R24".to_string(),
            severity: Severity::Error,
            title: "rust-toolchain.toml missing".to_string(),
            message: "No rust-toolchain.toml found at workspace root".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }

    // R55-R57: workspace metadata & release profile
    check_workspace_metadata(workspace_root, &mut results);

    results
}

pub fn check_per_crate_clippy(workspace_root: &Path, member_dirs: &[String]) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for member in member_dirs {
        let crate_dir = workspace_root.join(member);
        let crate_clippy = crate_dir.join("clippy.toml");
        if crate_clippy.exists() {
            results.push(CheckResult {
                id: "R2".to_string(),
                severity: Severity::Info,
                title: "Per-crate clippy.toml".to_string(),
                message: format!("Found for {member}"),
                file: Some(crate_clippy.display().to_string()),
                line: None,
            });

            // Check per-crate clippy.toml content for global-state type bans
            check_per_crate_clippy_content(&crate_clippy, member, &mut results);
        } else {
            results.push(CheckResult {
                id: "R2".to_string(),
                severity: Severity::Warn,
                title: "Per-crate clippy.toml missing".to_string(),
                message: format!("No clippy.toml for {member}"),
                file: Some(crate_dir.display().to_string()),
                line: None,
            });
        }
    }

    results
}

fn check_per_crate_clippy_content(
    path: &Path,
    member: &str,
    results: &mut Vec<CheckResult>,
) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let global_state_types = ["LazyLock", "OnceLock", "once_cell"];

    if let Some(types_arr) = table.get("disallowed-types").and_then(|v| v.as_array()) {
        let type_paths: Vec<String> = types_arr
            .iter()
            .filter_map(|v| {
                v.get("path")
                    .and_then(|p| p.as_str())
                    .or_else(|| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        let mut found_global_bans = Vec::new();
        for gs_type in &global_state_types {
            for tp in &type_paths {
                if tp.contains(gs_type) {
                    found_global_bans.push(tp.clone());
                }
            }
        }

        if found_global_bans.is_empty() {
            results.push(CheckResult {
                id: "R2".to_string(),
                severity: Severity::Info,
                title: format!("{member}: no global-state type bans"),
                message: "No LazyLock/OnceLock/once_cell bans in per-crate clippy.toml"
                    .to_string(),
                file: Some(path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "R2".to_string(),
                severity: Severity::Info,
                title: format!("{member}: global-state type bans present"),
                message: format!("Bans: {}", found_global_bans.join(", ")),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_clippy_thresholds(path: &Path, results: &mut Vec<CheckResult>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R3".to_string(),
                severity: Severity::Error,
                title: "clippy.toml unreadable".to_string(),
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
                id: "R3".to_string(),
                severity: Severity::Error,
                title: "clippy.toml parse error".to_string(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let expected: &[(&str, i64)] = &[
        ("too-many-lines-threshold", 75),
        ("cognitive-complexity-threshold", 15),
        ("too-many-arguments-threshold", 7),
        ("type-complexity-threshold", 75),
        ("max-struct-bools", 3),
    ];

    for (key, expected_val) in expected {
        match table.get(key) {
            Some(toml::Value::Integer(v)) if *v == *expected_val => {
                results.push(CheckResult {
                    id: "R3".to_string(),
                    severity: Severity::Info,
                    title: format!("{key} correct"),
                    message: format!("{key} = {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            Some(toml::Value::Integer(v)) => {
                results.push(CheckResult {
                    id: "R3".to_string(),
                    severity: Severity::Error,
                    title: format!("{key} wrong value"),
                    message: format!("Expected {expected_val}, got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            Some(_) => {
                results.push(CheckResult {
                    id: "R3".to_string(),
                    severity: Severity::Error,
                    title: format!("{key} wrong type"),
                    message: format!("Expected integer {expected_val}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R3".to_string(),
                    severity: Severity::Error,
                    title: format!("{key} missing"),
                    message: format!("Expected {key} = {expected_val}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}

fn check_rustfmt_settings(path: &Path, results: &mut Vec<CheckResult>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R22".to_string(),
                severity: Severity::Warn,
                title: "rustfmt.toml unreadable".to_string(),
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
                id: "R22".to_string(),
                severity: Severity::Warn,
                title: "rustfmt.toml parse error".to_string(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let mut expected_keys: BTreeSet<String> = BTreeSet::new();

    let expected_strings: &[(&str, &str)] = &[("edition", "2024")];

    let expected_ints: &[(&str, i64)] = &[("max_width", 100), ("tab_spaces", 4)];

    let expected_bools: &[(&str, bool)] = &[
        ("use_field_init_shorthand", true),
        ("use_try_shorthand", true),
        ("reorder_imports", true),
        ("reorder_modules", true),
    ];

    for (key, expected_val) in expected_strings {
        expected_keys.insert((*key).to_string());
        match table.get(key) {
            Some(toml::Value::String(v)) if v == expected_val => {
                // Correct — no output needed for matching values
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected \"{expected_val}\", got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("Expected {key} = \"{expected_val}\""),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    for (key, expected_val) in expected_ints {
        expected_keys.insert((*key).to_string());
        match table.get(key) {
            Some(toml::Value::Integer(v)) if *v == *expected_val => {
                // Correct
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected {expected_val}, got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("Expected {key} = {expected_val}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    for (key, expected_val) in expected_bools {
        expected_keys.insert((*key).to_string());
        match table.get(key) {
            Some(toml::Value::Boolean(v)) if *v == *expected_val => {
                // Correct
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected {expected_val}, got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_string(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("Expected {key} = {expected_val}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // R23: Report extra settings not in expected set
    if let Some(tbl) = table.as_table() {
        for key in tbl.keys() {
            if !expected_keys.contains(key) {
                results.push(CheckResult {
                    id: "R23".to_string(),
                    severity: Severity::Info,
                    title: format!("rustfmt extra setting: {key}"),
                    message: format!("{key} = {}", tbl.get(key).map_or("?".to_string(), |v| v.to_string())),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}

fn check_toolchain_settings(path: &Path, results: &mut Vec<CheckResult>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R25".to_string(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml unreadable".to_string(),
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
                id: "R25".to_string(),
                severity: Severity::Warn,
                title: "rust-toolchain.toml parse error".to_string(),
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
                id: "R25".to_string(),
                severity: Severity::Info,
                title: "Toolchain channel correct".to_string(),
                message: "channel = \"stable\"".to_string(),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R25".to_string(),
                severity: Severity::Warn,
                title: "Toolchain channel not stable".to_string(),
                message: format!("channel = \"{other}\", expected \"stable\""),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R25".to_string(),
                severity: Severity::Warn,
                title: "Toolchain channel missing".to_string(),
                message: "Expected [toolchain] channel = \"stable\"".to_string(),
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
                        id: "R25".to_string(),
                        severity: Severity::Info,
                        title: format!("Component {expected} present"),
                        message: format!("{expected} in components list"),
                        file: Some(path.display().to_string()),
                        line: None,
                    });
                } else {
                    results.push(CheckResult {
                        id: "R25".to_string(),
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
                id: "R25".to_string(),
                severity: Severity::Warn,
                title: "No components list".to_string(),
                message: "Expected [toolchain] components = [\"clippy\", \"rustfmt\"]"
                    .to_string(),
                file: Some(path.display().to_string()),
                line: None,
            });
        }
    }
}

// R55-R57: workspace metadata & release profile
fn check_workspace_metadata(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let cargo_path = workspace_root.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let content = match std::fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // R55: Report workspace edition and rust-version
    let edition = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("edition"))
        .and_then(|e| e.as_str())
        .or_else(|| {
            table
                .get("package")
                .and_then(|p| p.get("edition"))
                .and_then(|e| e.as_str())
        });

    let rust_version = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("rust-version"))
        .and_then(|r| r.as_str())
        .or_else(|| {
            table
                .get("package")
                .and_then(|p| p.get("rust-version"))
                .and_then(|r| r.as_str())
        });

    let mut meta_parts = Vec::new();
    if let Some(ed) = edition {
        meta_parts.push(format!("edition = {ed}"));
    }
    if let Some(rv) = rust_version {
        meta_parts.push(format!("rust-version = {rv}"));
    }

    if !meta_parts.is_empty() {
        results.push(CheckResult {
            id: "R55".to_string(),
            severity: Severity::Info,
            title: "Workspace metadata".to_string(),
            message: meta_parts.join(", "),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    }

    // R56: Report workspace publish status
    let publish = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("publish"))
        .or_else(|| {
            table.get("package").and_then(|p| p.get("publish"))
        });

    if let Some(p) = publish {
        results.push(CheckResult {
            id: "R56".to_string(),
            severity: Severity::Info,
            title: "Publish status".to_string(),
            message: format!("publish = {p}"),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    }

    // R57: Release profile
    let release = table
        .get("profile")
        .and_then(|p| p.get("release"));

    if let Some(rel) = release {
        if let Some(rel_table) = rel.as_table() {
            let settings: Vec<String> = rel_table
                .iter()
                .map(|(k, v)| format!("{k} = {v}"))
                .collect();
            results.push(CheckResult {
                id: "R57".to_string(),
                severity: Severity::Info,
                title: "Release profile".to_string(),
                message: settings.join(", "),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
    }
}
