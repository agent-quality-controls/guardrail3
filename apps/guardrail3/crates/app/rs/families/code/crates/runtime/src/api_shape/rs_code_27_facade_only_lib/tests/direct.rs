use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_27_facade_only_lib::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_private_use_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "use crate::internal::Thing;", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "lib.rs should stay facade-only",
            "lib.rs contains private use `crate::internal::Thing`. Keep lib.rs limited to facade declarations and type/const definitions.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_inline_public_module_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub mod api { pub fn run() {} }", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "lib.rs should stay facade-only",
            "lib.rs contains inline module `api`. Keep lib.rs limited to facade declarations and type/const definitions.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_test_inline_module_in_library_lib_rs() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg(test)]\npub mod tests { pub fn run() {} }",
        false,
    );

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "lib.rs should stay facade-only",
            "lib.rs contains inline module `tests`. Keep lib.rs limited to facade declarations and type/const definitions.",
            Some("src/lib.rs"),
            Some(2),
            false,
        )],
    );
}
