use guardrail3_app_rs_family_deny_assertions::rs_deny_04_deprecated_advisories as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn local_deprecated_advisory_fields_only_warn_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    let local = build_fixture_deny_toml("service")
        .replace("[advisories]\n", "[advisories]\nvulnerability = \"deny\"\n");
    write_file(tmp.path(), "apps/devctl/deny.toml", &local);

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "deprecated advisory field `vulnerability`",
            "`apps/devctl/deny.toml` uses deprecated `[advisories].vulnerability`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
