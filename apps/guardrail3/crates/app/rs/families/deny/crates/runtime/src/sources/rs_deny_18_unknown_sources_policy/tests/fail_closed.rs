use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_18_unknown_sources_policy as assertions;

use super::helpers::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_sources_section_is_missing() {
    let results = super::helpers::run_check(&remove_section(
        &build_fixture_deny_toml("service"),
        "sources",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[sources] section missing",
            "`deny.toml` has no `[sources]` section.",
            "deny.toml",
            false,
        )],
    );
}
