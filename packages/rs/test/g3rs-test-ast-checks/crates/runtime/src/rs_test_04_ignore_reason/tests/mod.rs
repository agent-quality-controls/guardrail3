use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_missing_ignore_reason() {
    let results = run_input(input(
        vec![file(
            "tests/slow.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[ignore]\nfn slow() {}\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-04",
        G3Severity::Error,
        "ignored test lacks reason",
        "tests/slow.rs",
        Some(2),
    );
}

#[test]
fn inventories_clean_file_without_ignored_tests() {
    let results = run_input(input(
        vec![file(
            "tests/ok.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn ok() {}\n",
        )],
        None,
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-04",
        "ignored tests have reasons",
        "tests/ok.rs",
    );
}
