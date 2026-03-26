use super::super::check_cycle_for_test as check_cycle;
use guardrail3_domain_report::Severity;

#[test]
fn same_layer_cycle_reports_exact_layer_and_path_chain() {
    let results = check_cycle(
        "domain",
        vec![
            "apps/api/crates/domain/a",
            "apps/api/crates/domain/b",
            "apps/api/crates/domain/c",
        ],
    );

    assert_eq!(results.len(), 1, "expected one cycle error: {results:#?}");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file, None);
    assert!(
        results[0]
            .title
            .contains("same-layer domain dependency cycle")
    );
    assert!(results[0].message.contains(
        "apps/api/crates/domain/a -> apps/api/crates/domain/b -> apps/api/crates/domain/c"
    ));
}
