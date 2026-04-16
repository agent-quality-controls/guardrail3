use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-01";

pub fn assert_forbidden_dependency(results: &[G3CheckResult], source_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("depends on forbidden crate")
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
                && result.title().contains("depends only on allowed layers")
                && result.file() == Some(source_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}
