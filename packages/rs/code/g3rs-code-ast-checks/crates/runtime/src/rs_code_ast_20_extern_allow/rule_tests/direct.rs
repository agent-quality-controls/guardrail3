use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_20_extern_allow::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_allow_attr_on_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("allow on extern block"),
            file: Some("src/ffi.rs"),
            inventory: Some(false),
            message: Some(
                "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_cfg_attr_allow_on_extern_block() {
    let content = r#"
#[cfg_attr(feature = "ffi", allow(improper_ctypes))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("allow on extern block"),
            file: Some("src/ffi.rs"),
            inventory: Some(false),
            message: Some(
                "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_multiple_lints_from_one_cfg_attr_allow_on_extern_block() {
    let content = r#"
#[cfg_attr(feature = "ffi", allow(improper_ctypes, clippy::all))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("allow on extern block"),
                file: Some("src/ffi.rs"),
                inventory: Some(false),
                message: Some(
                    "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression.",
                ),
                line: Some(2),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("allow on extern block"),
                file: Some("src/ffi.rs"),
                inventory: Some(false),
                message: Some(
                    "`#[cfg_attr(..., allow(clippy::all))]` on an `extern` block hides FFI risk behind a broad suppression.",
                ),
                line: Some(2),
            },
        ],
    );
}
