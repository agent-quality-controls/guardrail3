use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use guardrail3_domain_report::Severity;
use crate::test_support::copy_fixture;

#[test]
fn unparsable_ports_source_warns_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
        "pub trait Repo {\n",
    )
    .expect("write broken ports source");

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected one source-analysis warning: {warnings:#?}"
    );
    assert_eq!(warnings[0].severity, Severity::Warn);
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo/src/lib.rs")
    );
    assert!(
        warnings[0]
            .message
            .contains("Failed to parse Rust source file")
    );
}

#[test]
fn parse_failure_takes_precedence_over_impl_heavy_warning() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
        "mod extra;\npub trait Repo {\n",
    )
    .expect("write broken ports source");
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/extra.rs"),
        "pub struct ExtraA;\nimpl ExtraA { pub fn new() -> Self { Self } }\npub struct ExtraB;\nimpl ExtraB { pub fn new() -> Self { Self } }\n",
    )
    .expect("write impl-heavy extra module");

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "parse failure should short-circuit to one source-analysis warning: {warnings:#?}"
    );
    assert!(
        warnings[0].title.contains("source analysis failed"),
        "parse failure should not also emit impl-heavy warning: {warnings:#?}"
    );
    assert!(
        !warnings[0].title.contains("impl-heavy"),
        "parse failure should suppress the generic impl-heavy warning: {warnings:#?}"
    );
}
