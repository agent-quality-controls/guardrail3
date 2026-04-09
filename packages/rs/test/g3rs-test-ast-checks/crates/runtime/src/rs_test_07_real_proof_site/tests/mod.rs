use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_test_without_real_proof_site() {
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
        "RS-TEST-07",
        G3Severity::Error,
        "test lacks real proof site",
        "tests/missing.rs",
        Some(2),
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
        "RS-TEST-07",
        "real proof site present",
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
        "RS-TEST-07",
        "real proof site present",
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
        "RS-TEST-07",
        "real proof site present",
        "tests/glob.rs",
    );
}
