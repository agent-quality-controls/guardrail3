use super::super::{run_source_case, SourceCrateLayerForTest};

#[test]
fn impl_heavy_ports_warns() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        1,
        3,
        None,
        None,
    );

    assert_eq!(
        results.len(),
        1,
        "expected one trait-dominance warning: {results:#?}"
    );
}

#[test]
fn equal_impl_and_public_trait_counts_do_not_warn() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        2,
        2,
        None,
        None,
    );

    assert!(
        results.is_empty(),
        "expected no warning when impls and public traits are balanced: {results:#?}"
    );
}

#[test]
fn dto_only_ports_crate_stays_clean() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        0,
        0,
        None,
        None,
    );

    assert!(
        results.is_empty(),
        "expected DTO-only ports crates to stay clean: {results:#?}"
    );
}

#[test]
fn non_ports_crates_are_ignored() {
    let results = run_source_case(
        SourceCrateLayerForTest::Adapters,
        "api-adapters-http",
        "apps/api/crates/adapters/http",
        0,
        99,
        None,
        None,
    );

    assert!(
        results.is_empty(),
        "expected non-ports crates to be ignored by rule 22: {results:#?}"
    );
}
