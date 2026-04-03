use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_27_facade_only_lib::assert_findings;

#[test]
fn errors_on_private_use_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "use crate::internal::Thing;", false);

    // RS-CODE-27 retired: redundant with RS-ARCH-02.
    assert_findings(&results, &[]);
}

#[test]
fn errors_on_inline_public_module_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub mod api { pub fn run() {} }", false);

    // RS-CODE-27 retired: redundant with RS-ARCH-02.
    assert_findings(&results, &[]);
}

#[test]
fn errors_on_cfg_test_inline_module_in_library_lib_rs() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg(test)]\npub mod tests { pub fn run() {} }",
        false,
    );

    // RS-CODE-27 retired: redundant with RS-ARCH-02.
    assert_findings(&results, &[]);
}
