use guardrail3_app_rs_family_deny_assertions::rs_deny_29_ignore_accumulation as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_advisory_ignores, write_file};

#[test]
fn local_large_ignore_list_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_advisory_ignores(
            &build_fixture_deny_toml("service"),
            ["A", "B", "C", "D", "E", "F"]
                .into_iter()
                .map(|id| toml::Value::String(id.to_owned()))
                .collect(),
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`apps/devctl/deny.toml` has 6 `[advisories].ignore` entries (threshold: 5).",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
