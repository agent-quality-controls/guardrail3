use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_09_wildcards_inventory as assertions;

use crate::inputs::ConfigDenyInput;
use super::super::check;
use super::helpers::{build_fixture_deny_toml, config_facts, remove_section_key, set_section_string};

#[test]
fn warns_when_wildcards_is_missing_or_project_specific() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "bans",
        "wildcards",
    ));
    let custom = config_facts(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "wildcards",
        "deny",
    ));

    for (config, expected) in [
        (&missing, "`deny.toml` sets `[bans].wildcards = <missing>`."),
        (&custom, "`deny.toml` sets `[bans].wildcards = deny`."),
    ] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::warn(
                "wildcards differs from baseline",
                expected,
                "deny.toml",
                false,
            )],
        );
    }
}
