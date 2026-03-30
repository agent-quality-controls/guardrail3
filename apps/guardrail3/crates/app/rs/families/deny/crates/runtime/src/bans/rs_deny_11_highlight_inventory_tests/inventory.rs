use guardrail3_app_rs_family_deny_assertions::rs_deny_11_highlight_inventory as assertions;

use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{build_fixture_deny_toml, config_facts, remove_section_key, set_section_string};

#[test]
fn inventories_missing_or_project_specific_highlight() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "bans",
        "highlight",
    ));
    let custom = config_facts(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "highlight",
        "simplest",
    ));

    for (config, expected) in [
        (&missing, "`deny.toml` sets `[bans].highlight = <missing>`."),
        (&custom, "`deny.toml` sets `[bans].highlight = simplest`."),
    ] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::info(
                "highlight differs from baseline",
                expected,
                "deny.toml",
                true,
            )],
        );
    }
}
