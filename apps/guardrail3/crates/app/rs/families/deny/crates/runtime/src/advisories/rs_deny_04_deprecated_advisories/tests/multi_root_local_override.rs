use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_04_deprecated_advisories as assertions;

use super::helpers::build_fixture_deny_toml;

#[test]
fn local_deprecated_advisory_fields_only_warn_for_the_owned_local_root() {
    let local = build_fixture_deny_toml("service")
        .replace("[advisories]\n", "[advisories]\nvulnerability = \"deny\"\n");
    let results = super::helpers::run_check(&local);
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "deprecated advisory field `vulnerability`",
            "`deny.toml` uses deprecated `[advisories].vulnerability`.",
            "deny.toml",
            false,
        )],
    );
}
