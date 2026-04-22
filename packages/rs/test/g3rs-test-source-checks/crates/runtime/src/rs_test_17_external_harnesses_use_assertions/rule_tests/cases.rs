use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_17_external_harnesses_use_assertions::rule as assertions;

#[test]
fn reports_external_harness_direct_assertions() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/direct.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "#[test]\nfn direct() { assert_eq!(1, 1); }\n",
        )],
        Some("demo_assertions"),
    ));

    assertions::assert_has_result(
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
    let results = assertions::check(&assertions::input(
        vec![
            assertions::file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "pub fn assert_demo() { assert_eq!(1, 1); }\n",
            ),
            assertions::file(
                "tests/harness.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "use demo_assertions::assert_demo;\n#[test]\nfn harness() { assert_demo(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-17",
        "external harness uses owned assertions",
        "tests/harness.rs",
    );
}

#[test]
fn inventories_qualified_owned_assertions_call_even_with_local_same_name_helper() {
    let results = assertions::check(&assertions::input(
        vec![
            assertions::file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "pub fn assert_demo() { assert_eq!(1, 1); }\n",
            ),
            assertions::file(
                "tests/qualified.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "fn assert_demo() { assert_eq!(1, 1); }\n#[test]\nfn qualified() { demo_assertions::assert_demo(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-17",
        "external harness uses owned assertions",
        "tests/qualified.rs",
    );
}

#[test]
fn reports_external_harness_using_self_qualified_local_assertion_wrapper() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/self_wrapper.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "fn assert_demo() { assert_eq!(1, 1); }\nfn wrapper() { self::assert_demo(); }\n#[test]\nfn harness() { wrapper(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/self_wrapper.rs",
        Some(4),
    );
}

#[test]
fn reports_external_harness_using_crate_qualified_local_assertion_wrapper() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/crate_wrapper.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "fn assert_demo() { assert_eq!(1, 1); }\nfn wrapper() { crate::assert_demo(); }\n#[test]\nfn harness() { wrapper(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/crate_wrapper.rs",
        Some(4),
    );
}

#[test]
fn reports_external_harness_using_super_qualified_local_assertion_wrapper() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/super_wrapper.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "fn assert_demo() { assert_eq!(1, 1); }\nfn wrapper() { super::assert_demo(); }\n#[test]\nfn harness() { wrapper(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/super_wrapper.rs",
        Some(4),
    );
}

#[test]
fn reports_external_harness_using_local_assertion_helper() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/helper.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "fn local_assertion_helper() { assert_eq!(1, 1); }\n#[test]\nfn harness() { local_assertion_helper(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-17",
        G3Severity::Error,
        "external harness asserts directly",
        "tests/helper.rs",
        Some(3),
    );
}
