use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-arch/crate-has-facade";

pub fn assert_facade_inventory(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title() == "crate has facade entry point"
                && result.file() == Some(cargo_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_missing_facade(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "crate missing facade entry point"
                && result.file() == Some(cargo_file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}
