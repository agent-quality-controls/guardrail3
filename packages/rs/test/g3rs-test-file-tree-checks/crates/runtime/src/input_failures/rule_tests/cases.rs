use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::input_failures::rule as assertions;
use g3rs_test_ingestion_runtime::fixtures::file_tree::{component, file, input};

#[test]
fn reports_parse_failure_as_error_result() {
    let mut results = Vec::new();

    super::super::check(
        "demo",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
        &mut results,
    );

    assertions::assert_has_result(
        &results,
        "g3rs-test/filetree-input-failures",
        G3Severity::Error,
        "failed to read test input",
        "tests/broken.rs",
        None,
    );
    assertions::assert_message(
        &results,
        "g3rs-test/filetree-input-failures",
        "failed to read test input",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
    );
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
    input
        .input_failures
        .push(g3rs_test_types::G3RsTestFileTreeInputFailure {
            rel_path: "test_support/src/broken.rs".to_owned(),
            message: "parse failed".to_owned(),
        });

    let results = assertions::check(&input);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn inactive_root_with_only_assertions_module_stays_quiet() {
    let results = assertions::check(&input(
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

    assertions::assert_no_rule(&results, "g3rs-test/owned-sidecar-shape");
    assertions::assert_no_rule(&results, "g3rs-test/runtime-assertions-split");
    assertions::assert_no_rule(&results, "g3rs-test/filetree-input-failures");
    assertions::assert_no_rule(&results, "g3rs-test/test-support-generic");
}
