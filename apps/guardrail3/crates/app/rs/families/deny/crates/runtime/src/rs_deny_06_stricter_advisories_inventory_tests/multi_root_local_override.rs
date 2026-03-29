use guardrail3_app_rs_family_deny_assertions::rs_deny_06_stricter_advisories_inventory as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_section_string, write_file};

#[test]
fn local_stricter_advisory_value_only_inventories_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_string(
            &build_fixture_deny_toml("service"),
            "advisories",
            "unmaintained",
            "deny",
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "advisories `unmaintained` stricter than baseline",
            "`apps/devctl/deny.toml` sets `[advisories].unmaintained = \"deny\"`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
