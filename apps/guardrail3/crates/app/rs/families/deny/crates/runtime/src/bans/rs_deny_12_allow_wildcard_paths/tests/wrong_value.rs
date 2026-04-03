use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_12_allow_wildcard_paths as assertions;

use crate::inputs::ConfigDenyInput;
use super::super::check;
use super::helpers::{build_fixture_deny_toml, config_facts, remove_section_key, set_section_bool};

#[test]
fn errors_when_allow_wildcard_paths_is_missing_or_false() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "bans",
        "allow-wildcard-paths",
    ));
    let wrong = config_facts(&set_section_bool(
        &build_fixture_deny_toml("service"),
        "bans",
        "allow-wildcard-paths",
        false,
    ));

    for config in [&missing, &wrong] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::error(
                "allow-wildcard-paths must be true",
                "`deny.toml` must set `[bans].allow-wildcard-paths = true`.",
                "deny.toml",
                false,
            )],
        );
    }
}
