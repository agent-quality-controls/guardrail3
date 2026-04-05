use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

use crate::inputs::cargo_edition;

const ID: &str = "RS-FMT-02";

pub(crate) fn check(input: &G3FmtContentChecksInput, results: &mut Vec<GrdzCheckResult>) {
    let expected_edition = cargo_edition(&input.cargo).unwrap_or("2024");

    check_string(
        &input.rustfmt_rel_path,
        "edition",
        input.rustfmt.edition.as_deref(),
        expected_edition,
        results,
    );
    check_string(
        &input.rustfmt_rel_path,
        "style_edition",
        input.rustfmt.style_edition.as_deref(),
        expected_edition,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "max_width",
        input.rustfmt.max_width.map(i64::from),
        100,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "tab_spaces",
        input.rustfmt.tab_spaces.map(i64::from),
        4,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "use_field_init_shorthand",
        input.rustfmt.use_field_init_shorthand,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "use_try_shorthand",
        input.rustfmt.use_try_shorthand,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "reorder_imports",
        input.rustfmt.reorder_imports,
        true,
        results,
    );
    check_bool(
        &input.rustfmt_rel_path,
        "reorder_modules",
        input.rustfmt.reorder_modules,
        true,
        results,
    );
}

fn check_string(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<&str>,
    expected: &str,
    results: &mut Vec<GrdzCheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

fn check_int(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<i64>,
    expected: i64,
    results: &mut Vec<GrdzCheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

fn check_bool(
    rustfmt_rel_path: &str,
    key: &str,
    actual: Option<bool>,
    expected: bool,
    results: &mut Vec<GrdzCheckResult>,
) {
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => push_wrong(rustfmt_rel_path, key, actual, expected, results),
        None => push_missing(rustfmt_rel_path, key, expected, results),
    }
}

fn push_wrong(
    rel: &str,
    key: &str,
    actual: impl std::fmt::Display,
    expected: impl std::fmt::Display,
    results: &mut Vec<GrdzCheckResult>,
) {
    results.push(GrdzCheckResult::new(
        ID.to_owned(),
        GrdzSeverity::Warn,
        format!("rustfmt {key} wrong"),
        format!("{key} = {actual} but expected {expected}. Update {key} in rustfmt.toml."),
        Some(rel.to_owned()),
        None,
    ));
}

fn push_missing(
    rel: &str,
    key: &str,
    expected: impl std::fmt::Display,
    results: &mut Vec<GrdzCheckResult>,
) {
    results.push(GrdzCheckResult::new(
        ID.to_owned(),
        GrdzSeverity::Warn,
        format!("rustfmt {key} missing"),
        format!("{key} must be set to {expected}"),
        Some(rel.to_owned()),
        None,
    ));
}
