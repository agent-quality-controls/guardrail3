use g3rs_deny_config_checks_assertions::extra_deny_bans_inventory as assertions;

use test_support::run;

use super::helpers;

#[test]
fn inventories_canonical_baseline_as_having_no_extra_bans() {
    let results = run(
        helpers::service_canonical_bans_toml(),
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::extra_deny_bans_inventory::check,
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
