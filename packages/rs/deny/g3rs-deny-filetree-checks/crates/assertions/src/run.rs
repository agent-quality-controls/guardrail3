use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_combined_coverage_and_shadowing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-deny/coverage"
                && result.severity() == G3Severity::Info
                && result.title() == "workspace root covered by deny config"
                && result.message() == "workspace root `.` is covered by `deny.toml`."
                && result.inventory()
                && result.file() == Some("deny.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-deny/shadowing"
                && result.severity() == G3Severity::Error
                && result.title() == "multiple deny configs at one policy root"
                && result.message()
                    == "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml."
                && !result.inventory()
                && result.file() == Some(".cargo/deny.toml")
        }),
        "{results:#?}"
    );
}
