use guardrail3_app_rs_family_deny_assertions::rs_deny_23_skip_hygiene as assertions;

use super::super::{add_skip_entry, build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn local_skip_warning_only_hits_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &add_skip_entry(
            &build_fixture_deny_toml("service"),
            toml::Value::Table(toml::map::Map::from_iter([
                (
                    "crate".to_owned(),
                    toml::Value::String("plain-crate".to_owned()),
                ),
                (
                    "reason".to_owned(),
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ])),
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "skip entry",
                "`apps/devctl/deny.toml` has documented skip entry `plain-crate`.",
                "apps/devctl/deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "skip entry count",
                "`apps/devctl/deny.toml` has 1 skip entries (1 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
