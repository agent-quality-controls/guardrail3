use std::fmt::Display;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-02";

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "rustfmt config parse error".to_owned(),
            message: "rustfmt config exists but could not be parsed as TOML".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        });
        return;
    };

    check_string(
        parsed,
        rel,
        "edition",
        input.workspace_edition.unwrap_or("2024"),
        results,
    );
    check_int(parsed, rel, "max_width", 100, results);
    check_int(parsed, rel, "tab_spaces", 4, results);
    check_bool(parsed, rel, "use_field_init_shorthand", true, results);
    check_bool(parsed, rel, "use_try_shorthand", true, results);
    check_bool(parsed, rel, "reorder_imports", true, results);
    check_bool(parsed, rel, "reorder_modules", true, results);
}

fn check_string(
    table: &toml::Value,
    rel: &str,
    key: &str,
    expected: &str,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key).and_then(toml::Value::as_str) {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rel, key, actual, expected, results),
        None => push_missing(rel, key, expected, results),
    }
}

fn check_int(
    table: &toml::Value,
    rel: &str,
    key: &str,
    expected: i64,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key).and_then(toml::Value::as_integer) {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rel, key, actual, expected, results),
        None => push_missing(rel, key, expected, results),
    }
}

fn check_bool(
    table: &toml::Value,
    rel: &str,
    key: &str,
    expected: bool,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key).and_then(toml::Value::as_bool) {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rel, key, actual, expected, results),
        None => push_missing(rel, key, expected, results),
    }
}

fn push_wrong(
    rel: &str,
    key: &str,
    actual: impl Display,
    expected: impl Display,
    results: &mut Vec<CheckResult>,
) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("rustfmt {key} wrong"),
        message: format!("{key} = {actual} but expected {expected}"),
        file: Some(rel.to_owned()),
        line: None,
        inventory: false,
    });
}

fn push_missing(rel: &str, key: &str, expected: impl Display, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("rustfmt {key} missing"),
        message: format!("{key} must be set to {expected}"),
        file: Some(rel.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_fmt_02_settings_tests.rs"]
mod tests;
