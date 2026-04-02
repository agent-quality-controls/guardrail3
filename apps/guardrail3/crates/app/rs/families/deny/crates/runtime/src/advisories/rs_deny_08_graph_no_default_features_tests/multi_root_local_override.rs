use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_08_graph_no_default_features as assertions;

use super::super::{build_fixture_deny_toml, set_section_bool};

#[test]
fn local_graph_no_default_features_drift_only_errors_for_the_owned_local_root() {
    let results = super::super::run_check(&set_section_bool(
        &build_fixture_deny_toml("service"),
        "graph",
        "no-default-features",
        true,
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph no-default-features must be false",
            "`deny.toml` must set `[graph].no-default-features = false`.",
            "deny.toml",
            false,
        )],
    );
}
