use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_05_should_panic_expected::rule as assertions;

#[test]
fn reports_missing_should_panic_expected_string() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/panic.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[should_panic]\nfn panics() { panic!(\"boom\"); }\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-05",
        G3Severity::Error,
        "should_panic missing expected string",
        "tests/panic.rs",
        Some(2),
    );
}

#[test]
fn inventories_explicit_should_panic_expected_string() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/panic.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[should_panic(expected = \"boom\")]\nfn panics() { panic!(\"boom\"); }\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-05",
        "should_panic expected string present",
        "tests/panic.rs",
    );
}

#[test]
fn reports_blank_should_panic_expected_string() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/panic.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[should_panic(expected = \"   \")]\nfn panics() { panic!(\"boom\"); }\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-05",
        G3Severity::Error,
        "should_panic missing expected string",
        "tests/panic.rs",
        Some(2),
    );
}

#[test]
fn inventories_cfg_attr_should_panic_with_expected_string() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/panic.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\n#[cfg_attr(test, should_panic(expected = \"boom\"))]\nfn panics() { panic!(\"boom\"); }\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-05",
        "should_panic expected string present",
        "tests/panic.rs",
    );
}
