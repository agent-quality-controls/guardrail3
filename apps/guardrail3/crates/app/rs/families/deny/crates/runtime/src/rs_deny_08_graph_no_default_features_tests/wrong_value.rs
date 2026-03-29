use guardrail3_app_rs_family_deny_assertions::rs_deny_08_graph_no_default_features as assertions;

use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{build_fixture_deny_toml, config_facts, remove_section_key, set_section_bool};

#[test]
fn errors_when_no_default_features_is_missing_or_true() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "graph",
        "no-default-features",
    ));
    let wrong = config_facts(&set_section_bool(
        &build_fixture_deny_toml("service"),
        "graph",
        "no-default-features",
        true,
    ));

    for config in [&missing, &wrong] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::error(
                "graph no-default-features must be false",
                "`deny.toml` must set `[graph].no-default-features = false`.",
                "deny.toml",
                false,
            )],
        );
    }
}
