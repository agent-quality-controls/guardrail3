use super::super::super::test_support::{copy_fixture, run_family};
use guardrail3_domain_report::Severity;

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

#[test]
fn parse_failure_takes_precedence_over_public_trait_violation() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/lib.rs"),
        "mod extra;\npub trait Broken {\n",
    )
    .expect("write broken adapter source");
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/extra.rs"),
        "pub trait ExtraBoundary {}\n",
    )
    .expect("write public-trait extra module");

    let results = run_family(tmp.path());
    let errors: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-23")
        .collect();
    assert_eq!(
        errors.len(),
        1,
        "parse failure should short-circuit to one source-analysis error: {errors:#?}"
    );
    assert!(
        errors[0].title.contains("source analysis failed"),
        "parse failure should not also emit public-trait violation: {errors:#?}"
    );
    assert!(
        !errors[0].title.contains("defines public traits"),
        "parse failure should suppress the public-trait rule body: {errors:#?}"
    );
}
