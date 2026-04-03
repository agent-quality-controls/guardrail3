use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_23_skip_hygiene as assertions;

use super::helpers::{add_skip_entry, build_fixture_deny_toml};

#[test]
fn local_skip_warning_only_hits_the_owned_local_root() {
    let results = super::helpers::run_check(&add_skip_entry(
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
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "skip entry",
                "`deny.toml` has documented skip entry `plain-crate`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "skip entry count",
                "`deny.toml` has 1 skip entries (1 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
