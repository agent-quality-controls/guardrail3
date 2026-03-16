use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

type ExpectedStr<'a> = (&'a str, &'a str);
type ExpectedInt<'a> = (&'a str, i64);
type ExpectedBool<'a> = (&'a str, bool);
#[allow(clippy::too_many_lines)] // reason: rustfmt settings validation
pub fn check_rustfmt_settings(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let content = match fs.read_file_err(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R22".to_owned(),
                severity: Severity::Warn,
                title: "rustfmt.toml unreadable".to_owned(),
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
                id: "R22".to_owned(),
                severity: Severity::Warn,
                title: "rustfmt.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let mut expected_keys: BTreeSet<String> = BTreeSet::new();
    let expected_strings: &[ExpectedStr<'_>] = &[("edition", "2024")];
    let expected_ints: &[ExpectedInt<'_>] = &[("max_width", 100), ("tab_spaces", 4)];

    let expected_bools: &[ExpectedBool<'_>] = &[
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
                    message: format!("{key} = {v} but should be \"{expected_val}\". Set `{key} = \"{expected_val}\"` in rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("{key} not set. Add `{key} = \"{expected_val}\"` to rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
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
                    message: format!("{key} = {v} but should be {expected_val}. Set `{key} = {expected_val}` in rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("{key} not set. Add `{key} = {expected_val}` to rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
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
                    message: format!("{key} = {v} but should be {expected_val}. Set `{key} = {expected_val}` in rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R22".to_owned(),
                    severity: Severity::Warn,
                    title: format!("rustfmt {key} missing"),
                    message: format!("{key} not set. Add `{key} = {expected_val}` to rustfmt.toml."),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
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
                            .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
                    ),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }
}
