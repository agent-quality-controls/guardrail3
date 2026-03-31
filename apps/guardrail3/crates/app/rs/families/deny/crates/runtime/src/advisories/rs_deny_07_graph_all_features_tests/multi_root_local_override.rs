use guardrail3_app_rs_family_deny_assertions::rs_deny_07_graph_all_features as assertions;

use super::super::{build_fixture_deny_toml, set_section_bool};

#[test]
fn local_graph_all_features_drift_only_errors_for_the_owned_local_root() {
    let results = super::super::run_check(&set_section_bool(
        &build_fixture_deny_toml("service"),
        "graph",
        "all-features",
        false,
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph all-features must be true",
            "`deny.toml` must set `[graph].all-features = true`.",
            "deny.toml",
            false,
        )],
    );
}
