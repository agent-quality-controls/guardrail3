use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_assertions_module_without_proof_bearing_export() {
    let results = run_input(input(
        vec![file(
            "assertions/src/lib.rs",
            G3RsTestFileKind::AssertionsModule,
            Some("demo_assertions"),
            "pub fn helper() { let _x = 1; }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-16",
        G3Severity::Error,
        "assertions module lacks proof-bearing export",
        "assertions/src/lib.rs",
        Some(1),
    );
}

#[test]
fn inventories_assertions_module_with_real_assertions() {
    let results = run_input(input(
        vec![file(
            "assertions/src/lib.rs",
            G3RsTestFileKind::AssertionsModule,
            Some("demo_assertions"),
            "pub fn helper() { assert_eq!(1, 1); }\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-16",
        "assertions module proves runtime",
        "assertions/src/lib.rs",
    );
}

#[test]
fn reports_sidecar_owning_semantic_result_assertions() {
    let results = run_input(input(
        vec![file(
            "src/feature_tests/mod.rs",
            G3RsTestFileKind::InternalSidecarMod,
            Some("demo_assertions"),
            "#[test]\nfn sidecar() {\n    let result = CheckResult::new(String::new(), Severity::Info, String::new(), String::new(), None, None);\n    assert_eq!(result.id(), \"\");\n}\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_result(
        &results,
        "RS-TEST-SOURCE-16",
        G3Severity::Error,
        "sidecar owns semantic result assertion",
        "src/feature_tests/mod.rs",
        Some(2),
    );
}

#[test]
fn inventories_sidecar_delegating_semantic_proof() {
    let results = run_input(input(
        vec![file(
            "src/feature_tests/mod.rs",
            G3RsTestFileKind::InternalSidecarMod,
            Some("demo_assertions"),
            "#[test]\nfn sidecar() {\n    let result = returns_result().expect(\"should fail\");\n    assert_demo(result);\n}\n",
        )],
        Some("demo_assertions"),
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-16",
        "sidecar delegates semantic proof",
        "src/feature_tests/mod.rs",
    );
}
