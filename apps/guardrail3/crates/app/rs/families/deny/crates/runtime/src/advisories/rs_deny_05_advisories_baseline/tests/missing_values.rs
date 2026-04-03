use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_05_advisories_baseline as assertions;

use super::helpers::{build_fixture_deny_toml, remove_section_key};

#[test]
fn errors_when_baseline_advisory_values_are_missing() {
    let deny = remove_section_key(
        &remove_section_key(
            &build_fixture_deny_toml("service"),
            "advisories",
            "unmaintained",
        ),
        "advisories",
        "yanked",
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisories `unmaintained` missing",
                "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "advisories `yanked` missing",
                "`deny.toml` must set `[advisories].yanked = \"warn\"`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
