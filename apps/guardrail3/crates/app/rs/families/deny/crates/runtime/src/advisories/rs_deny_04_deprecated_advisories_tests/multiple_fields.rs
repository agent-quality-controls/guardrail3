use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_04_deprecated_advisories as assertions;

use super::super::{build_fixture_deny_toml, set_section_string};

#[test]
fn warns_once_per_deprecated_advisory_field() {
    let deny = set_section_string(
        &set_section_string(
            &set_section_string(
                &build_fixture_deny_toml("service"),
                "advisories",
                "vulnerability",
                "deny",
            ),
            "advisories",
            "notice",
            "warn",
        ),
        "advisories",
        "unsound",
        "deny",
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "deprecated advisory field `notice`",
                "`deny.toml` uses deprecated `[advisories].notice`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "deprecated advisory field `unsound`",
                "`deny.toml` uses deprecated `[advisories].unsound`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "deprecated advisory field `vulnerability`",
                "`deny.toml` uses deprecated `[advisories].vulnerability`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
