use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/types-purity";

pub fn assert_impure_dependency(results: &[G3CheckResult], source_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("depends on impure external crate")
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
                && result.title().contains("stays pure")
                && result.file() == Some(source_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_policy_error_contains(results: &[G3CheckResult], source_file: &str, needle: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("cannot validate purity")
                && result.file() == Some(source_file)
                && !result.inventory()
                && result.message().contains(needle)
        }),
        "{results:#?}"
    );
}
