use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_config_04_graph_all_features as assertions;

use crate::inputs::ConfigDenyInput;
use super::super::check;
use super::helpers::{build_fixture_deny_toml, config_facts, remove_section_key, set_section_bool};

#[test]
fn errors_when_all_features_is_missing_or_false() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "graph",
        "all-features",
    ));
    let wrong = config_facts(&set_section_bool(
        &build_fixture_deny_toml("service"),
        "graph",
        "all-features",
        false,
    ));

    for config in [&missing, &wrong] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::error(
                "graph all-features must be true",
                "`deny.toml` must set `[graph].all-features = true`.",
                "deny.toml",
                false,
            )],
        );
    }
}
