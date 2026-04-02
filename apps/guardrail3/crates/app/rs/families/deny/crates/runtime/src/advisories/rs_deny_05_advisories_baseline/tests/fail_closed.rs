use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_05_advisories_baseline as assertions;

use super::super::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_advisories_section_is_missing() {
    let results = super::super::run_check(&remove_section(
        &build_fixture_deny_toml("service"),
        "advisories",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[advisories] section missing",
            "`deny.toml` has no `[advisories]` section.",
            "deny.toml",
            false,
        )],
    );
}
