use super::super::super::test_support::{copy_fixture, run_family};
use crate::domain::report::Severity;

#[test]
fn unparsable_adapter_source_errors_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/lib.rs"),
        "pub trait Broken {\n",
    )
    .expect("write broken adapter source");

    let results = run_family(tmp.path());
    let errors: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-23")
        .collect();
    assert_eq!(
        errors.len(),
        1,
        "expected one source-analysis error: {errors:#?}"
    );
    assert_eq!(errors[0].severity, Severity::Error);
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/backend/crates/adapters/outbound/postgres/src/lib.rs")
    );
    assert!(
        errors[0]
            .message
            .contains("Failed to parse Rust source file")
    );
}
