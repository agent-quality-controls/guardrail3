use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

type ExpectedStr<'a> = (&'a str, &'a str);
type ExpectedInt<'a> = (&'a str, i64);
type ExpectedBool<'a> = (&'a str, bool);

pub fn check_rustfmt_settings(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let content = match fs.read_file_err(path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R22".to_owned(),
                severity: Severity::Error,
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
                severity: Severity::Error,
                title: "rustfmt.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let expected_keys = check_rustfmt_expected_values(&table, path, results);
    check_rustfmt_extra_settings(&table, path, &expected_keys, results);
}

fn check_rustfmt_expected_values(
    table: &toml::Value,
    path: &Path,
    results: &mut Vec<CheckResult>,
) -> BTreeSet<String> {
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
        check_rustfmt_str(table, key, expected_val, path, results);
    }

    for (key, expected_val) in expected_ints {
        let _ = expected_keys.insert((*key).to_owned());
        check_rustfmt_int(table, key, *expected_val, path, results);
    }

    for (key, expected_val) in expected_bools {
        let _ = expected_keys.insert((*key).to_owned());
        check_rustfmt_bool(table, key, *expected_val, path, results);
    }

    expected_keys
}

fn check_rustfmt_str(
    table: &toml::Value,
    key: &str,
    expected_val: &str,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key) {
        Some(toml::Value::String(v)) if v == expected_val => {}
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
                message: format!(
                    "{key} not set. Add `{key} = \"{expected_val}\"` to rustfmt.toml."
                ),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

/// Check a rustfmt.toml setting against an expected value.
/// Works for both integer and boolean settings by comparing the TOML value's display form.
fn check_rustfmt_setting(
    table: &toml::Value,
    key: &str,
    expected_display: &str,
    matches: impl Fn(&toml::Value) -> bool,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key) {
        Some(v) if matches(v) => {}
        Some(v) => {
            results.push(CheckResult {
                id: "R22".to_owned(),
                severity: Severity::Warn,
                title: format!("rustfmt {key} wrong"),
                message: format!(
                    "{key} = {v} but should be {expected_display}. Set `{key} = {expected_display}` in rustfmt.toml."
                ),
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
                message: format!(
                    "{key} not set. Add `{key} = {expected_display}` to rustfmt.toml."
                ),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_rustfmt_int(
    table: &toml::Value,
    key: &str,
    expected_val: i64,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let expected_display = expected_val.to_string();
    check_rustfmt_setting(
        table,
        key,
        &expected_display,
        |v| v.as_integer() == Some(expected_val),
        path,
        results,
    );
}

fn check_rustfmt_bool(
    table: &toml::Value,
    key: &str,
    expected_val: bool,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let expected_display = expected_val.to_string();
    check_rustfmt_setting(
        table,
        key,
        &expected_display,
        |v| v.as_bool() == Some(expected_val),
        path,
        results,
    );
}

fn check_rustfmt_extra_settings(
    table: &toml::Value,
    path: &Path,
    expected_keys: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
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
