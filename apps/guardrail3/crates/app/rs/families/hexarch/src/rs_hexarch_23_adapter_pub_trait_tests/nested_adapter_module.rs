use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn nested_adapter_module_with_public_trait_errors() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "mod nested;\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/nested/mod.rs",
        "pub trait NestedBoundary {\n}\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-23");
    assert_eq!(
        errors.len(),
        1,
        "expected one adapter public-trait error from nested module: {errors:#?}"
    );
    assert_eq!(errors[0].severity, Severity::Error);
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/backend/crates/adapters/outbound/postgres")
    );
}
