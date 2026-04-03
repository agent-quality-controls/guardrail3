use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_26_lib_glob_reexport::assert_findings;

#[test]
fn warns_on_pub_use_glob_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub use crate::inner::*;", false);

    // RS-CODE-26 retired: redundant with RS-ARCH-02.
    assert_findings(&results, &[]);
}

#[test]
fn warns_on_grouped_pub_use_glob_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub use crate::inner::{Visible, *};", false);

    // RS-CODE-26 retired: redundant with RS-ARCH-02.
    assert_findings(&results, &[]);
}
