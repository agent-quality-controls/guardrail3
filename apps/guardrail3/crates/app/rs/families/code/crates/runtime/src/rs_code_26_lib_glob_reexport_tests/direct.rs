use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_26_lib_glob_reexport::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn warns_on_pub_use_glob_in_library_lib_rs() {
    let content = "pub use crate::inner::*;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-26");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "glob re-export in lib.rs");
    assert_eq!(
        results[0].message,
        "`pub use crate::inner::*` creates an unstable API surface."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_grouped_pub_use_glob_in_library_lib_rs() {
    let content = "pub use crate::inner::{Visible, *};";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-26");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "glob re-export in lib.rs");
    assert_eq!(
        results[0].message,
        "`pub use crate::inner::*` creates an unstable API surface."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
