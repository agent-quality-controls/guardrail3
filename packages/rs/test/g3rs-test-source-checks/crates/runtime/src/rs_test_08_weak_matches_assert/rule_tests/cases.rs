use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_08_weak_matches_assert::rule as assertions;

#[test]
fn reports_weak_matches_assertions() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/matches.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn weak() { assert!(matches!(Some(1), Some(_))); }\n",
        )],
        None,
    ));

    assertions::assert_has_result(
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
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/matches.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn strong() { assert!(matches!(Some(1), Some(1))); }\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-08",
        "weak matches assertion absent",
        "tests/matches.rs",
    );
}

#[test]
fn reports_assert_matches_and_debug_assert_wildcards() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/weak.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn weak() {\n    assert_matches!(Some(1), Some(_));\n    debug_assert!(matches!(Some(1), Some(_)));\n}\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-08",
        G3Severity::Error,
        "weak matches assertion",
        "tests/weak.rs",
        Some(3),
    );
    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-08",
        G3Severity::Error,
        "weak matches assertion",
        "tests/weak.rs",
        Some(4),
    );
}
