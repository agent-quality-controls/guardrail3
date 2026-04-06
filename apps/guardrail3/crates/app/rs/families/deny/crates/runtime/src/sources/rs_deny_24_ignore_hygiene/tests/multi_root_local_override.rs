use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_19_ignore_hygiene as assertions;

use super::helpers::{build_fixture_deny_toml, set_advisory_ignores};

#[test]
fn local_advisory_ignore_warning_only_hits_the_owned_local_root() {
    let results = super::helpers::run_check(&set_advisory_ignores(
        &build_fixture_deny_toml("service"),
        vec![toml::Value::Table(toml::map::Map::from_iter([
            (
                "id".to_owned(),
                toml::Value::String("RUSTSEC-2026-0000".to_owned()),
            ),
            (
                "reason".to_owned(),
                toml::Value::String("good enough reason text".to_owned()),
            ),
        ]))],
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "advisory ignore entry",
                "`deny.toml` has documented advisory ignore `RUSTSEC-2026-0000`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (1 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
