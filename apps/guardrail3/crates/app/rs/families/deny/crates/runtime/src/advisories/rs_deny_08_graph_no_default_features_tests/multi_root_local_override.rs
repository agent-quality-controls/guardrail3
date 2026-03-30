use guardrail3_app_rs_family_deny_assertions::rs_deny_08_graph_no_default_features as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_section_bool, write_file};

#[test]
fn local_graph_no_default_features_drift_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_bool(
            &build_fixture_deny_toml("service"),
            "graph",
            "no-default-features",
            true,
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph no-default-features must be false",
            "`apps/devctl/deny.toml` must set `[graph].no-default-features = false`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
