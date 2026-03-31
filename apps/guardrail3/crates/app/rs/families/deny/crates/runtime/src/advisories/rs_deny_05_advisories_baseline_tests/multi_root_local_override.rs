use guardrail3_app_rs_family_deny_assertions::rs_deny_05_advisories_baseline as assertions;

use super::super::{build_fixture_deny_toml, set_section_string};

#[test]
fn local_wrong_advisory_value_only_errors_for_the_owned_local_root() {
    let results = super::super::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "advisories",
        "yanked",
        "deny",
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `yanked` has wrong value",
            "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `deny`.",
            "deny.toml",
            false,
        )],
    );
}
