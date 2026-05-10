use guardrail3_check_types::{G3CheckResult, G3Severity};

/// I D const.
const ID: &str = "g3rs-arch/feature-gated-exports";

/// Internal.
///
/// # Panics
///
/// See body for assertions.
pub fn assert_ungated_exports(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "facade exports not feature-gated"
                && result.file() == Some(file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

/// Internal.
///
/// # Panics
///
/// See body for assertions.
pub fn assert_gated_inventory(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title() == "facade exports properly feature-gated"
                && result.file() == Some(file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}

/// Internal.
///
/// # Panics
///
/// See body for assertions.
pub fn assert_all_gated_exports(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "`all` feature must not directly gate exports"
                && result.file() == Some(file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}
