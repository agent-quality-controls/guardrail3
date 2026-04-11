use guardrail3_check_types::G3Severity;
use g3rs_test_types::G3RsTestFileKind;

use crate::test_helpers::{assert_no_rule, component, file, input, run_input};

#[test]
fn reports_parse_failure_as_error_result() {
    let mut results = Vec::new();

    crate::rs_test_10_input_failures::check(
        "demo",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
        &mut results,
    );

    assert_eq!(results.len(), 1, "{results:#?}");
    let result = &results[0];
    assert_eq!(result.id(), "RS-TEST-10");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "failed to read test input");
    assert_eq!(result.file(), Some("tests/broken.rs"));
    assert_eq!(result.message(), "expected one of: `fn`, `struct`, `enum`");
}

#[test]
fn inactive_root_with_only_test_support_stays_quiet() {
    let mut input = input(
        vec![file(
            "test_support/src/lib.rs",
            G3RsTestFileKind::TestSupport,
            None,
            Some("lib"),
            None,
            "pub fn helper(name: &str) -> String { name.to_owned() }\n",
        )],
        vec![component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        )],
    );
    input.input_failures.push(g3rs_test_types::G3RsTestFileTreeInputFailure {
        rel_path: "test_support/src/broken.rs".to_owned(),
        message: "parse failed".to_owned(),
    });

    let results = run_input(input);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn inactive_root_with_only_assertions_module_stays_quiet() {
    let results = run_input(input(
        vec![file(
            "crates/assertions/src/lib.rs",
            G3RsTestFileKind::AssertionsModule,
            Some(""),
            Some("lib"),
            Some("demo_assertions"),
            "pub fn assert_runtime() { assert!(true); }\n",
        )],
        vec![component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        )],
    ));

    assert_no_rule(&results, "RS-TEST-02");
    assert_no_rule(&results, "RS-TEST-03");
    assert_no_rule(&results, "RS-TEST-10");
    assert_no_rule(&results, "RS-TEST-18");
}
