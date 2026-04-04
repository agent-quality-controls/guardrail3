use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::helpers::build_fixture_deny_toml;

#[test]
fn inventories_canonical_deny_baseline_as_having_no_extra_bans() {
    let results = super::helpers::run_check(&build_fixture_deny_toml("service"));

    assert!(
        assertions::findings(&results).iter().any(|finding| {
            finding.title == "no extra deny bans"
                && finding.file == Some("deny.toml")
                && finding.message == "`deny.toml` has 0 deny bans beyond the managed baseline."
        }),
        "expected canonical deny baseline inventory: {results:#?}"
    );
}
