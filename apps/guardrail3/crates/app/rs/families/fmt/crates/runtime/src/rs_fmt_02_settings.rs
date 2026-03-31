use std::fmt::Display;

use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::CargoEditionState;
use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-02";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel.as_deref() else {
        return;
    };
    let Some(parsed) = input.parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rustfmt config parse error".to_owned(),
            "rustfmt config exists but could not be parsed as a TOML table".to_owned(),
            Some(rel.to_owned()),
            None,
            false,
        ));
        return;
    };
    if !parsed.is_table() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rustfmt config parse error".to_owned(),
            "rustfmt config exists but could not be parsed as a TOML table".to_owned(),
            Some(rel.to_owned()),
            None,
            false,
        ));
        return;
    };

    check_string(
        parsed,
        rel,
        "edition",
        match &input.cargo_edition {
            CargoEditionState::Present(edition) => edition,
            CargoEditionState::MissingManifest
            | CargoEditionState::ParseError
            | CargoEditionState::MissingEdition => "2024",
        },
        results,
    );
    check_string(
        parsed,
        rel,
        "style_edition",
        match &input.cargo_edition {
            CargoEditionState::Present(edition) => edition,
            CargoEditionState::MissingManifest
            | CargoEditionState::ParseError
            | CargoEditionState::MissingEdition => "2024",
        },
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
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        format!("rustfmt {key} wrong"),
        format!("{key} = {actual} but expected {expected}"),
        Some(rel.to_owned()),
        None,
        false,
    ));
}

fn push_missing(rel: &str, key: &str, expected: impl Display, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        format!("rustfmt {key} missing"),
        format!("{key} must be set to {expected}"),
        Some(rel.to_owned()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_check(parsed: Option<toml::Value>) -> Vec<CheckResult> {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed,
        escape_hatches: Vec::new(),
        cargo_edition: CargoEditionState::Present("2024".to_owned()),
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_fmt_02_settings_tests/mod.rs"]
mod rs_fmt_02_settings_tests;
