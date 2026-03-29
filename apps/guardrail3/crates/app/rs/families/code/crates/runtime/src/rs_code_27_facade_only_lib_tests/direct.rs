use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_27_facade_only_lib::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_private_use_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "use crate::internal::Thing;", false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "lib.rs should stay facade-only",
            message: "lib.rs contains private use `crate::internal::Thing`. Keep lib.rs limited to facade declarations and type/const definitions.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_inline_public_module_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub mod api { pub fn run() {} }", false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "lib.rs should stay facade-only",
            message: "lib.rs contains inline module `api`. Keep lib.rs limited to facade declarations and type/const definitions.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
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
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "lib.rs should stay facade-only",
            message: "lib.rs contains inline module `tests`. Keep lib.rs limited to facade declarations and type/const definitions.",
            file: Some("src/lib.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}
