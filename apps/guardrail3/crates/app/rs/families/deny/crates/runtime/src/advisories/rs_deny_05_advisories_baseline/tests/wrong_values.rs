use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_config_02_advisories_baseline as assertions;

use super::helpers::{build_fixture_deny_toml, set_section_string};

#[test]
fn errors_when_advisories_baseline_is_weakened() {
    let deny = set_section_string(
        &set_section_string(
            &build_fixture_deny_toml("service"),
            "advisories",
            "unmaintained",
            "allow",
        ),
        "advisories",
        "yanked",
        "allow",
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisories `unmaintained` has wrong value",
                "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`, found `allow`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "advisories `yanked` has wrong value",
                "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
