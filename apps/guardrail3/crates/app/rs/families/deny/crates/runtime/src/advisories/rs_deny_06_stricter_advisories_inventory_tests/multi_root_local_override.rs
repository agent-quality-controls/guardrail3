use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_06_stricter_advisories_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_section_string};

#[test]
fn local_stricter_advisory_value_only_inventories_for_the_owned_local_root() {
    let results = super::super::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "advisories",
        "unmaintained",
        "deny",
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "advisories `unmaintained` stricter than baseline",
            "`deny.toml` sets `[advisories].unmaintained = \"deny\"`.",
            "deny.toml",
            true,
        )],
    );
}
