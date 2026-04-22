use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-CONFIG-01";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "rustfmt config unreadable".to_owned(),
                "rustfmt config exists but could not be read from disk".to_owned(),
                Some(input.rustfmt_rel_path.clone()),
                None,
            ));
            return;
        }
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "rustfmt config parse error".to_owned(),
                "rustfmt config exists but could not be parsed as a TOML table".to_owned(),
                Some(input.rustfmt_rel_path.clone()),
                None,
            ));
            return;
        }
    };
    let expected_edition = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo.edition.as_deref(),
        g3rs_fmt_types::G3RsFmtCargoState::Missing
        | g3rs_fmt_types::G3RsFmtCargoState::Unreadable
        | g3rs_fmt_types::G3RsFmtCargoState::ParseError => None,
    }
    .unwrap_or("2024");

    check_string(
        &input.rustfmt_rel_path,
        "edition",
        rustfmt.edition.as_deref(),
        expected_edition,
        results,
    );
    check_string(
        &input.rustfmt_rel_path,
        "style_edition",
        rustfmt.style_edition.as_deref(),
        expected_edition,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "max_width",
        rustfmt.max_width,
        100,
        results,
    );
    check_int(
        &input.rustfmt_rel_path,
        "tab_spaces",
        rustfmt.tab_spaces,
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
