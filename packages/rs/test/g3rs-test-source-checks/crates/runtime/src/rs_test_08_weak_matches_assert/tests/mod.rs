use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_weak_matches_assertions() {
    let results = run_input(input(
        vec![file(
            "tests/matches.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn weak() { assert!(matches!(Some(1), Some(_))); }\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-08",
        G3Severity::Error,
        "weak matches assertion",
        "tests/matches.rs",
        Some(2),
    );
}

#[test]
fn inventories_specific_matches_assertions() {
    let results = run_input(input(
        vec![file(
            "tests/matches.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn strong() { assert!(matches!(Some(1), Some(1))); }\n",
        )],
        None,
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-08",
        "weak matches assertion absent",
        "tests/matches.rs",
    );
}

#[test]
fn reports_assert_matches_and_debug_assert_wildcards() {
    let results = run_input(input(
        vec![file(
            "tests/weak.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn weak() {\n    assert_matches!(Some(1), Some(_));\n    debug_assert!(matches!(Some(1), Some(_)));\n}\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-08",
        G3Severity::Error,
        "weak matches assertion",
        "tests/weak.rs",
        Some(3),
    );
    assert_has_result(
        &results,
        "RS-TEST-SOURCE-08",
        G3Severity::Error,
        "weak matches assertion",
        "tests/weak.rs",
        Some(4),
    );
}
