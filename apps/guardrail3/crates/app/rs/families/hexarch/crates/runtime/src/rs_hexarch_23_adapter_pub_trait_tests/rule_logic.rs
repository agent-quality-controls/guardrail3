use super::super::run_source_case;
use guardrail3_domain_report::Severity;

#[test]
fn adapter_public_trait_errors() {
    let results = run_source_case(
        "api-adapter-http",
        "apps/api/crates/adapters/http",
        1,
        0,
        None,
        None,
    );

    assert_eq!(
        results.len(),
        1,
        "expected one adapter pub-trait error: {results:#?}"
    );
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].file.as_deref(),
        Some("apps/api/crates/adapters/http")
    );
}
