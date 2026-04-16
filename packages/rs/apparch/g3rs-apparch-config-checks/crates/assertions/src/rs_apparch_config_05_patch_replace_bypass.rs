use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-05";

pub fn assert_missing_waiver(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("missing waiver")
                && result.file() == Some("Cargo.toml")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_documented_patch(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Warn
                && result.title().contains("is documented")
                && result.file() == Some("Cargo.toml")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_weak_reason(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("reason too weak")
                && result.file() == Some("Cargo.toml")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_policy_error_contains(results: &[G3CheckResult], needle: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title().contains("cannot validate")
                && result.file() == Some("Cargo.toml")
                && !result.inventory()
                && result.message().contains(needle)
        }),
        "{results:#?}"
    );
}
