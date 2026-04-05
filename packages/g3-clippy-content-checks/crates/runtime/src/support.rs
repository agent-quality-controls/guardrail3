use clippy_toml_parser::ClippyToml;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn check_threshold(
    id: &str,
    clippy_rel_path: &str,
    key: &str,
    actual: Option<u64>,
    expected: i64,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(actual) if i64::try_from(actual).ok() == Some(expected) => {
            results.push(
                G3CheckResult::new(
                    id.to_owned(),
                    G3Severity::Info,
                    format!("{key} correct"),
                    format!("{key} = {expected}"),
                    Some(clippy_rel_path.to_owned()),
                    None,
                )
                .into_inventory(),
            );
        }
        Some(actual) => {
            results.push(G3CheckResult::new(
                id.to_owned(),
                G3Severity::Error,
                format!("{key} wrong value"),
                format!(
                    "Expected {expected}, got {actual}. Set `{key} = {expected}` in clippy.toml."
                ),
                Some(clippy_rel_path.to_owned()),
                None,
            ));
        }
        None => {
            results.push(G3CheckResult::new(
                id.to_owned(),
                G3Severity::Error,
                format!("{key} missing"),
                format!("Add `{key} = {expected}` to clippy.toml."),
                Some(clippy_rel_path.to_owned()),
                None,
            ));
        }
    }
}

pub(crate) fn relaxation_message(
    key: &str,
    expected: bool,
    actual: Option<bool>,
) -> String {
    let policy = match key {
        "allow-dbg-in-tests" | "allow-print-in-tests" => {
            "Tests should stay quiet and deterministic."
        }
        "allow-expect-in-tests" => {
            "Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`."
        }
        "allow-panic-in-tests" => "panic!() must remain banned in tests.",
        "allow-unwrap-in-tests" => "unwrap() must remain banned in tests.",
        _ => "Managed test relaxation keys must match the hardened clippy policy.",
    };

    match actual {
        Some(actual) => format!("`{key}` must be `{expected}`; found `{actual}`. {policy}"),
        None => format!("`{key}` must be set explicitly to `{expected}`. {policy}"),
    }
}

pub(crate) const fn allow_dbg_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_dbg_in_tests
}

pub(crate) const fn allow_print_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_print_in_tests
}

pub(crate) const fn allow_expect_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_expect_in_tests
}

pub(crate) const fn allow_panic_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_panic_in_tests
}

pub(crate) const fn allow_unwrap_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_unwrap_in_tests
}
