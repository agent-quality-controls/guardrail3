use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_test_checking_results_through_local_path() {
    let results = run_input(input(
        vec![file(
            "src/feature_tests/golden.rs",
            G3RsTestFileKind::InternalSidecarSupport,
            Some("demo_assertions"),
            "use super::assertions;\n#[test]\nfn golden() { assertions::assert_results(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-07",
        G3Severity::Error,
        "test checks results through local path",
        "src/feature_tests/golden.rs",
        Some(3),
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-SOURCE-07"
                && result.title() == "test checks results through local path"
                && result.file() == Some("src/feature_tests/golden.rs")
        })
        .expect("missing RS-TEST-SOURCE-07 result");
    assert_eq!(
        result.message(),
        "Test `golden` in `src/feature_tests/golden.rs` checks results through local path `super::assertions::assert_results`. Move those result assertions into the shared assertions crate and call that from the test instead, so internal and external tests use the same proof."
    );
}

#[test]
fn reports_test_without_shared_proof_step() {
    let results = run_input(input(
        vec![file(
            "tests/missing.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "#[test]\nfn missing() { let _value = 1; }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-07",
        G3Severity::Error,
        "test has no shared proof step",
        "tests/missing.rs",
        Some(2),
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-SOURCE-07"
                && result.title() == "test has no shared proof step"
                && result.file() == Some("tests/missing.rs")
        })
        .expect("missing RS-TEST-SOURCE-07 result");
    assert_eq!(
        result.message(),
        "Test `missing` in `tests/missing.rs` does not call the shared assertions crate. Move the result assertions into the shared assertions crate and call that from the test, so internal and external tests use the same proof."
    );
}

#[test]
fn ignores_local_setup_helper_when_test_has_no_proof() {
    let results = run_input(input(
        vec![file(
            "src/feature_tests/missing.rs",
            G3RsTestFileKind::InternalSidecarSupport,
            Some("demo_assertions"),
            "use super::helpers::git_init;\n#[test]\nfn missing() { git_init(); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-07",
        G3Severity::Error,
        "test has no shared proof step",
        "src/feature_tests/missing.rs",
        Some(3),
    );
}

#[test]
fn inventories_assertion_macro_proof() {
    let results = run_input(input(
        vec![file(
            "tests/ok.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("demo_assertions"),
            "#[test]\nfn ok() { assert_eq!(1, 1); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-07",
        "test uses shared proof",
        "tests/ok.rs",
    );
}

#[test]
fn inventories_owned_assertions_proof_via_alias_import() {
    let results = run_input(input(
        vec![
            file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "pub fn assert_demo() { assert_eq!(1, 1); }\n",
            ),
            file(
                "tests/alias.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "use demo_assertions::assert_demo as prove;\n#[test]\nfn alias() { prove(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-07",
        "test uses shared proof",
        "tests/alias.rs",
    );
}

#[test]
fn inventories_owned_assertions_proof_via_glob_import() {
    let results = run_input(input(
        vec![
            file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "pub fn assert_demo() { assert_eq!(1, 1); }\n",
            ),
            file(
                "tests/glob.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "use demo_assertions::*;\n#[test]\nfn glob() { assert_demo(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-07",
        "test uses shared proof",
        "tests/glob.rs",
    );
}

#[test]
fn inventories_owned_assertions_wrapper_over_other_assertions_crate() {
    let results = run_input(input(
        vec![
            file(
                "assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some("demo_assertions"),
                "use other_assertions::assert_demo;\npub fn prove() { assert_demo(); }\n",
            ),
            file(
                "tests/wrapper.rs",
                G3RsTestFileKind::ExternalHarness,
                Some("demo_assertions"),
                "use demo_assertions::prove;\n#[test]\nfn wrapper() { prove(); }\n",
            ),
        ],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-07",
        "test uses shared proof",
        "tests/wrapper.rs",
    );
}
