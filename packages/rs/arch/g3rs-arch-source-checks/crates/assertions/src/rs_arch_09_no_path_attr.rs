use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-SOURCE-09";

pub fn assert_no_findings(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(|result| result.id() != ID),
        "{results:#?}"
    );
}

pub fn assert_path_attr_error(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "#[path] attribute forbidden"
                && result.file() == Some(file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}
