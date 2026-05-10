#![allow(
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by `io-traits-in-types`.
const ID: &str = "g3rs-apparch/io-traits-in-types";

pub fn assert_trait_violation(results: &[G3CheckResult], source_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("defines public trait")
                && result.file() == Some(source_file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_clean_inventory(results: &[G3CheckResult], source_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title().contains("defines no public traits")
                && result.file() == Some(source_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_no_findings(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(|result| result.id() != ID),
        "{results:#?}"
    );
}
