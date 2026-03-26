use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;
use guardrail3_domain_report::Severity;
use crate::test_support::{copy_fixture, write_file};

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-23");
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
