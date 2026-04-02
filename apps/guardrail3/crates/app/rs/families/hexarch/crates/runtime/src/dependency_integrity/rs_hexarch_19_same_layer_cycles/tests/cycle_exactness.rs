use super::super::check_cycle_for_test as check_cycle;
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_19_same_layer_cycles as assertions;
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_19_same_layer_cycles::Severity;

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

    assertions::assert_error_result_summary(
        &results,
        "",
        1,
        &[],
        Some(None),
        Some(Severity::Error),
        Some("same-layer domain dependency cycle"),
        Some("apps/api/crates/domain/a -> apps/api/crates/domain/b -> apps/api/crates/domain/c"),
    );
}
