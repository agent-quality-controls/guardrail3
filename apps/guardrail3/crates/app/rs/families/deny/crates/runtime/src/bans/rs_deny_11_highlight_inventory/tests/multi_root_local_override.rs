use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_07_highlight_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, set_section_string};

#[test]
fn local_highlight_drift_only_inventories_for_the_owned_local_root() {
    let results = super::helpers::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "highlight",
        "simplified",
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "highlight differs from baseline",
            "`deny.toml` sets `[bans].highlight = simplified`.",
            "deny.toml",
            true,
        )],
    );
}
