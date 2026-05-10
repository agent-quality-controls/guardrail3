#![allow(
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by `types-public-surface`.
const ID: &str = "g3rs-apparch/types-public-surface";

pub fn assert_behavior_violation(results: &[G3CheckResult], source_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("exposes behavioral API")
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
                && result
                    .title()
                    .contains("keeps public behavior out of its surface")
                && result.file() == Some(source_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}
