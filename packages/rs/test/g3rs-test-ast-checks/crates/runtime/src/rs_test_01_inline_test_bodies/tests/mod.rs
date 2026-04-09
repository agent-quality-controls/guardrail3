use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{assert_has_inventory, assert_has_result, file, input, run_input};

#[test]
fn reports_inline_cfg_test_body() {
    let results = run_input(input(
        vec![file(
            "src/lib.rs",
            G3RsTestFileKind::Source,
            None,
            "#[cfg(test)]\nmod tests { #[test] fn works() {} }\n",
        )],
        None,
    ));

    assert_has_result(
        &results,
        "RS-TEST-01",
        G3Severity::Error,
        "inline cfg(test) body in src",
        "src/lib.rs",
        Some(1),
    );
}

#[test]
fn inventories_sidecar_cfg_test_module() {
    let results = run_input(input(
        vec![file(
            "src/lib.rs",
            G3RsTestFileKind::Source,
            None,
            "#[cfg(test)]\nmod tests;\n",
        )],
        None,
    ));

    assert_has_inventory(
        &results,
        "RS-TEST-01",
        "inline cfg(test) body absent",
        "src/lib.rs",
    );
}
