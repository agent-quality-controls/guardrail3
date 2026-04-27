use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-arch/no-boundary-crossing";

pub fn assert_no_findings(results: &[G3CheckResult]) {
    assert!(
        results.iter().all(|result| result.id() != ID),
        "{results:#?}"
    );
}

pub fn assert_boundary_violation(results: &[G3CheckResult], cargo_file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "dependency crosses crate boundary"
                && result.file() == Some(cargo_file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}
