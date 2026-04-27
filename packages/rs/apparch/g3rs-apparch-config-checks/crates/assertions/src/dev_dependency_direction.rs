use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/dev-dependency-direction";

pub fn assert_direction_warning(
    results: &[G3CheckResult],
    source_file: &str,
    target: &str,
    kind_label: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Warn
                && result.title() == "dev-dependency direction violation"
                && result.file() == Some(source_file)
                && result.message().contains(&format!("crate `{target}`"))
                && result.message().contains(kind_label)
                && !result.inventory()
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
