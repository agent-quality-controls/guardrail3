use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_literal_only_assertions() {
    let results = run_input(input(
        vec![file(
            "tests/lit.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn tautology() { assert_eq!(1, 1); }\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-06",
        G3Severity::Error,
        "tautological assertion",
        "tests/lit.rs",
        Some(2),
    );
}

#[test]
fn inventories_real_assertions() {
    let results = run_input(input(
        vec![file(
            "tests/real.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn real() { let value = 1; assert_eq!(value, 1); }\n",
        )],
        None,
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-06",
        "tautological assertions absent",
        "tests/real.rs",
    );
}
