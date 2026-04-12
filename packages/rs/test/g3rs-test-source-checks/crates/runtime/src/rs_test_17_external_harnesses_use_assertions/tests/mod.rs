use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_external_harness_direct_assertions() {
    let results = run_input(input(
        vec![file(
            "tests/direct.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "#[test]\nfn direct() { assert_eq!(1, 1); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/direct.rs",
        Some(2),
    );
}

#[test]
fn inventories_external_harness_using_assertions_crate() {
    let results = run_input(input(
        vec![
            file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "pub fn assert_demo() { assert_eq!(1, 1); }\n",
            ),
            file(
                "tests/harness.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "use demo_assertions::assert_demo;\n#[test]\nfn harness() { assert_demo(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-17",
        "external harness uses owned assertions",
        "tests/harness.rs",
    );
}

#[test]
fn reports_external_harness_using_local_assertion_helper() {
    let results = run_input(input(
        vec![file(
            "tests/helper.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "fn local_assertion_helper() { assert_eq!(1, 1); }\n#[test]\nfn harness() { local_assertion_helper(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/helper.rs",
        Some(3),
    );
}
