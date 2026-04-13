use g3rs_deny_config_checks_assertions::rs_deny_config_26_extra_deny_bans_inventory as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn inventories_canonical_baseline_as_having_no_extra_bans() {
    let results = run(
        &canonical_bans_toml("service"),
        Some("service"),
        true,
        crate::rs_deny_config_26_extra_deny_bans_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no extra deny bans",
            "`deny.toml` has 0 deny bans beyond the managed baseline.",
            "deny.toml",
            true,
        )],
    );
}
