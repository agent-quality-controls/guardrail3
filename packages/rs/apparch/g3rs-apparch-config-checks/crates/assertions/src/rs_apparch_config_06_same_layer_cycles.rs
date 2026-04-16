use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-06";

pub fn assert_cycle(results: &[G3CheckResult], title_fragment: &str, needle: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains(title_fragment)
                && !result.inventory()
                && result.message().contains(needle)
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
