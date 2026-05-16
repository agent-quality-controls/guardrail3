use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::baseline::{
    ALLOW_DBG_IN_TESTS, ALLOW_EXPECT_IN_TESTS, ALLOW_PANIC_IN_TESTS, ALLOW_PRINT_IN_TESTS,
    ALLOW_UNWRAP_IN_TESTS,
};
use crate::support::{
    allow_dbg_in_tests, allow_expect_in_tests, allow_panic_in_tests, allow_print_in_tests,
    allow_unwrap_in_tests, relaxation_message, typed_clippy,
};

/// I D const.
const ID: &str = "g3rs-clippy/test-relaxations";

/// check fn.
pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(clippy) = typed_clippy(input) else {
        return;
    };
    let mut mismatch_count = 0usize;

    for (key, expected, actual, severity, title) in [
        (
            "allow-dbg-in-tests",
            ALLOW_DBG_IN_TESTS,
            allow_dbg_in_tests(clippy),
            G3Severity::Warn,
            "clippy test relaxation enabled",
        ),
        (
            "allow-print-in-tests",
            ALLOW_PRINT_IN_TESTS,
            allow_print_in_tests(clippy),
            G3Severity::Warn,
            "clippy test relaxation enabled",
        ),
        (
            "allow-expect-in-tests",
            ALLOW_EXPECT_IN_TESTS,
            allow_expect_in_tests(clippy),
            G3Severity::Error,
            "clippy test expect policy misconfigured",
        ),
        (
            "allow-panic-in-tests",
            ALLOW_PANIC_IN_TESTS,
            allow_panic_in_tests(clippy),
            G3Severity::Error,
            "clippy test panic relaxation enabled",
        ),
        (
            "allow-unwrap-in-tests",
            ALLOW_UNWRAP_IN_TESTS,
            allow_unwrap_in_tests(clippy),
            G3Severity::Error,
            "clippy test unwrap relaxation enabled",
        ),
    ] {
        if actual == Some(expected) {
            continue;
        }

        mismatch_count = mismatch_count.saturating_add(1);
        let suffix = if actual.is_some() { "" } else { " missing" };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            format!("{title}{suffix}"),
            relaxation_message(key, expected, actual),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }

    if mismatch_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "clippy test relaxation policy exact".to_owned(),
                "Managed test relaxation keys match the expected clippy policy.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
