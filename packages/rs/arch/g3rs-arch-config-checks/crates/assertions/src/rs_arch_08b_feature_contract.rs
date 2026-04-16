use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-CONFIG-08";

pub fn assert_no_findings(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(|result| result.id() != ID),
        "{results:#?}"
    );
}

pub fn assert_missing_default_feature(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "missing `default` feature"
                && result.file() == Some(cargo_file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_empty_default_feature(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "`default` feature is empty"
                && result.file() == Some(cargo_file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_feature_inventory(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title() == "feature contract supports facade exports"
                && result.file() == Some(cargo_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}
