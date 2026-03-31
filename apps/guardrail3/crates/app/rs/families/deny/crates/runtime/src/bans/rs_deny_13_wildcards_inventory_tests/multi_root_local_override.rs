use guardrail3_app_rs_family_deny_assertions::rs_deny_13_wildcards_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_section_string};

#[test]
fn local_wildcards_drift_only_warns_for_the_owned_local_root() {
    let results = super::super::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "wildcards",
        "warn",
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "wildcards differs from baseline",
            "`deny.toml` sets `[bans].wildcards = warn`.",
            "deny.toml",
            false,
        )],
    );
}
