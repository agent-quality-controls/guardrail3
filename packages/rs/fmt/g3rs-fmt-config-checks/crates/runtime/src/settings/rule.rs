use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::inputs::{cargo, cargo_edition, rustfmt, rustfmt_edition, rustfmt_style_edition};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-fmt/rustfmt-required-settings";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rustfmt) = rustfmt(input) else {
        emit_rustfmt_state_blocker(input, results);
        return;
    };
    let expected_edition = cargo(input).and_then(cargo_edition).unwrap_or("2024");

    check_string(
        &input.rustfmt_rel_path,
        "edition",
        rustfmt_edition(rustfmt.edition),
        expected_edition,
        results,
    );
    check_string(
        &input.rustfmt_rel_path,
        "style_edition",
        rustfmt_style_edition(rustfmt.style_edition),
        expected_edition,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "max_width",
        rustfmt.max_width.map(i64::from),
        100,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "tab_spaces",
        rustfmt.tab_spaces.map(i64::from),
        4,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "use_field_init_shorthand",
        rustfmt.use_field_init_shorthand,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "use_try_shorthand",
        rustfmt.use_try_shorthand,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "reorder_imports",
        rustfmt.reorder_imports,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "reorder_modules",
        rustfmt.reorder_modules,
        true,
        results,
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

/// Emits the blocker finding describing why the rustfmt config could not be read or parsed.
/// Returns silently when the rustfmt state is `Parsed` (the caller would have early-returned).
fn emit_rustfmt_state_blocker(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let (title, message) = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => (
            "rustfmt config unreadable".to_owned(),
            "rustfmt config exists but could not be read from disk".to_owned(),
        ),
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => (
            "rustfmt config parse error".to_owned(),
            "rustfmt config exists but could not be parsed as a TOML table".to_owned(),
        ),
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(_) => return,
    };
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        title,
        message,
        Some(input.rustfmt_rel_path.clone()),
        None,
    ));
}

/// Implements `check string`.
fn check_string(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<&str>,
    expected: &str,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

/// Implements `check int`.
fn check_int(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<i64>,
    expected: i64,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

/// Implements `check bool`.
fn check_bool(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<bool>,
    expected: bool,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

/// Implements `push wrong`.
fn push_wrong(
    rel: &str,
    key: &str,
    actual: impl std::fmt::Display,
    expected: impl std::fmt::Display,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        format!("rustfmt {key} wrong"),
        format!("{key} = {actual} but expected {expected}. Update {key} in rustfmt.toml."),
        Some(rel.to_owned()),
        None,
    ));
}

/// Implements `push missing`.
fn push_missing(
    rel: &str,
    key: &str,
    expected: impl std::fmt::Display,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        format!("rustfmt {key} missing"),
        format!("{key} must be set to {expected}"),
        Some(rel.to_owned()),
        None,
    ));
}
