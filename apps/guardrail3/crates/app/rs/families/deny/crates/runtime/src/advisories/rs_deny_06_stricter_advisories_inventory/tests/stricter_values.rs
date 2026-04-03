use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_06_stricter_advisories_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, set_section_string};

#[test]
fn inventories_each_advisory_key_that_is_stricter_than_baseline() {
    let deny = set_section_string(
        &set_section_string(
            &build_fixture_deny_toml("service"),
            "advisories",
            "unmaintained",
            "deny",
        ),
        "advisories",
        "yanked",
        "deny",
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "advisories `unmaintained` stricter than baseline",
                "`deny.toml` sets `[advisories].unmaintained = \"deny\"`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "advisories `yanked` stricter than baseline",
                "`deny.toml` sets `[advisories].yanked = \"deny\"`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
