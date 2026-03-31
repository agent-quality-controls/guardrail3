use guardrail3_app_rs_family_deny_assertions::rs_deny_29_ignore_accumulation as assertions;

use super::super::{build_fixture_deny_toml, set_advisory_ignores};

#[test]
fn local_large_ignore_list_only_warns_for_the_owned_local_root() {
    let results = crate::run_config_rule_for_test(
        &set_advisory_ignores(
        &build_fixture_deny_toml("service"),
        ["A", "B", "C", "D", "E", "F"]
            .into_iter()
            .map(|id| toml::Value::String(id.to_owned()))
            .collect(),
    ),
        None,
        super::super::check,
    );
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5).",
            "deny.toml",
            false,
        )],
    );
}
