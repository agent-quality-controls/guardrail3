use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_06_tautological_assertions::rule as assertions;

#[test]
fn reports_literal_only_assertions() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/lit.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn tautology() { assert_eq!(1, 1); }\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-06",
        G3Severity::Error,
        "tautological assertion",
        "tests/lit.rs",
        Some(2),
    );
}

#[test]
fn inventories_real_assertions() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "tests/real.rs",
            G3RsTestFileKind::ExternalHarness,
            None,
            "#[test]\nfn real() { let value = 1; assert_eq!(value, 1); }\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-06",
        "tautological assertions absent",
        "tests/real.rs",
    );
}
