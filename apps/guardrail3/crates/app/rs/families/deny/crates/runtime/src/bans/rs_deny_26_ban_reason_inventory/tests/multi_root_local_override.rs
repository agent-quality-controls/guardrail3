use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, remove_deny_ban_reason};

#[test]
fn removing_reason_from_canonical_ban_stays_quiet() {
    let results = super::helpers::run_check(&remove_deny_ban_reason(
        &build_fixture_deny_toml("service"),
        "json5",
    ));
    assert!(
        assertions::findings(&results)
            .iter()
            .any(|finding| finding.title == "no extra deny bans"),
        "missing reasons on deny-only entries should stay quiet: {results:#?}"
    );
}
