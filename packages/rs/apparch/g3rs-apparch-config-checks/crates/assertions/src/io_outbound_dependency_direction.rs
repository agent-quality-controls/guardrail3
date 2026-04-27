use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/io-outbound-dependency-direction";

pub fn assert_forbidden_dependency(results: &[G3CheckResult], source_file: &str, target: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title()
                    == format!("io/outbound crate `db` depends on forbidden crate `{target}`")
                && result.file() == Some(source_file)
                && result
                    .message()
                    .contains(&format!("dependency on `{target}`"))
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_clean_inventory(results: &[G3CheckResult], source_file: &str, source: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title()
                    == format!("io/outbound crate `{source}` depends only on allowed layers")
                && result.file() == Some(source_file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}
