use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_26_lib_glob_reexport::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_pub_use_glob_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub use crate::inner::*;", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "glob re-export in lib.rs",
            "`pub use crate::inner::*` creates an unstable API surface.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn warns_on_grouped_pub_use_glob_in_library_lib_rs() {
    let results = check_source("src/lib.rs", "pub use crate::inner::{Visible, *};", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "glob re-export in lib.rs",
            "`pub use crate::inner::*` creates an unstable API surface.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}
