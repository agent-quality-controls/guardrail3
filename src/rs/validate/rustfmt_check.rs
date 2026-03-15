use std::collections::BTreeSet;
use std::path::Path;

use crate::report::types::{CheckResult, Severity};

#[allow(clippy::too_many_lines)] // reason: rustfmt settings validation
#[allow(clippy::or_fun_call)] // reason: map_or with function call is intentional for display
pub fn check_rustfmt_settings(path: &Path, results: &mut Vec<CheckResult>) {
    let content = match crate::fs::read_file_err(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R22".to_owned(),
                severity: Severity::Warn,
                title: "rustfmt.toml unreadable".to_owned(),
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
                id: "R22".to_owned(),
                severity: Severity::Warn,
                title: "rustfmt.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let mut expected_keys: BTreeSet<String> = BTreeSet::new();
    #[allow(clippy::type_complexity)] // reason: legitimate complex type
    let expected_strings: &[(&str, &str)] = &[("edition", "2024")];
    #[allow(clippy::type_complexity)] // reason: legitimate complex type
    let expected_ints: &[(&str, i64)] = &[("max_width", 100), ("tab_spaces", 4)];

    #[allow(clippy::type_complexity)] // reason: legitimate type for expected settings
    let expected_bools: &[(&str, bool)] = &[
        ("use_field_init_shorthand", true),
        ("use_try_shorthand", true),
        ("reorder_imports", true),
        ("reorder_modules", true),
    ];

    for (key, expected_val) in expected_strings {
        let _ = expected_keys.insert((*key).to_owned());
        match table.get(key) {
            Some(toml::Value::String(v)) if v == expected_val => {
                // Correct — no output needed for matching values
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected \"{expected_val}\", got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
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
        let _ = expected_keys.insert((*key).to_owned());
        match table.get(key) {
            Some(toml::Value::Integer(v)) if *v == *expected_val => {
                // Correct
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected {expected_val}, got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
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
        let _ = expected_keys.insert((*key).to_owned());
        match table.get(key) {
            Some(toml::Value::Boolean(v)) if *v == *expected_val => {
                // Correct
            }
            Some(v) => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} wrong"),
                    message: format!("Expected {expected_val}, got {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
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
                    id: "R23".to_owned(),
                    severity: Severity::Info,
                    title: format!("rustfmt extra setting: {key}"),
                    message: format!(
                        "{key} = {}",
                        tbl.get(key)
                            .map_or("?".to_owned(), std::string::ToString::to_string)
                    ),
                    file: Some(path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}
