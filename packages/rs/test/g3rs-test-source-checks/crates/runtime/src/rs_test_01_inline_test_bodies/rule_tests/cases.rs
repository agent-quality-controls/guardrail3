use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_source_checks_assertions::rs_test_01_inline_test_bodies::rule as assertions;

#[test]
fn reports_inline_cfg_test_body() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "src/lib.rs",
            G3RsTestFileKind::Source,
            None,
            "#[cfg(test)]\nmod tests { #[test] fn works() {} }\n",
        )],
        None,
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-01",
        G3Severity::Error,
        "inline cfg(test) body in src",
        "src/lib.rs",
        Some(1),
    );
}

#[test]
fn inventories_sidecar_cfg_test_module() {
    let results = assertions::check(&assertions::input(
        vec![assertions::file(
            "src/lib.rs",
            G3RsTestFileKind::Source,
            None,
            "#[cfg(test)]\nmod tests;\n",
        )],
        None,
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-SOURCE-01",
        "inline cfg(test) body absent",
        "src/lib.rs",
    );
}
