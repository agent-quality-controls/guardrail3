use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_20_extern_allow::{
    RuleFinding, assert_errors_on_allow_attr_on_extern_block,
    assert_errors_on_cfg_attr_allow_covers_an_extern_block,
    assert_errors_on_mixed_allow_and_cfg_attr_on_the_same_extern_block,
    assert_errors_on_multiple_lints_from_one_cfg_attr_allow_on_extern_block,
    assert_errors_on_multiple_lints_from_one_extern_block_allow_attribute,
    assert_errors_when_stacked_allow_attrs_cover_the_same_extern_block, assert_findings,
};

#[test]
fn errors_on_allow_attr_on_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    assert_errors_on_allow_attr_on_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
    );
}

#[test]
fn errors_on_expect_attr_on_extern_block() {
    let content = r#"
#[expect(improper_ctypes)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let results = check_source("src/ffi.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "expect on extern block",
            "`#[expect(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression.",
            Some("src/ffi.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_multiple_lints_from_one_extern_block_allow_attribute() {
    let content = r#"
#[allow(improper_ctypes, clippy::all)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    assert_errors_on_multiple_lints_from_one_extern_block_allow_attribute(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
    );
}

#[test]
fn errors_when_stacked_allow_attrs_cover_the_same_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    assert_errors_when_stacked_allow_attrs_cover_the_same_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
        3,
    );
}

#[test]
fn errors_when_cfg_attr_allow_covers_an_extern_block() {
    let content = r#"
mod ffi_surface {
    #[cfg_attr(feature = "ffi", allow(improper_ctypes))]
    unsafe extern "C" {
        fn puts(s: *const i8);
    }
}
"#;
    assert_errors_on_cfg_attr_allow_covers_an_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        3,
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
    assert_errors_on_multiple_lints_from_one_cfg_attr_allow_on_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
    );
}

#[test]
fn errors_on_mixed_allow_and_cfg_attr_on_the_same_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
#[cfg_attr(feature = "ffi", allow(improper_ctypes_definitions))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    assert_errors_on_mixed_allow_and_cfg_attr_on_the_same_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
        3,
    );
}

#[test]
fn errors_on_nested_cfg_attr_allow_on_extern_block() {
    let content = r#"
#[cfg_attr(test, cfg_attr(unix, allow(improper_ctypes)))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    assert_errors_on_cfg_attr_allow_covers_an_extern_block(
        &check_source("src/ffi.rs", content, false),
        "src/ffi.rs",
        2,
    );
}
