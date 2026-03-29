use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_20_extern_allow::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_cfg_attr_expect_on_extern_block() {
    let content = r#"
#[cfg_attr(feature = "ffi", expect(improper_ctypes))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "expect on extern block",
            message: "`#[cfg_attr(..., expect(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some("src/ffi.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}

#[test]
fn errors_on_nested_cfg_attr_expect_on_extern_block() {
    let content = r#"
#[cfg_attr(test, cfg_attr(unix, expect(improper_ctypes)))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "expect on extern block",
            message: "`#[cfg_attr(..., expect(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
            file: Some("src/ffi.rs"),
            line: Some(2),
            inventory: false,
        }],
    );
}
