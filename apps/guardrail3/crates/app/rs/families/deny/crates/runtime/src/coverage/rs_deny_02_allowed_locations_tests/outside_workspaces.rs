use guardrail3_app_rs_family_deny_assertions::rs_deny_02_allowed_locations as assertions;

use super::super::check;
use super::super::{collected_facts, forbidden_input};
use test_support::{build_fixture_deny_toml, dir_entry, project_tree};

#[test]
fn reports_stray_deny_config_outside_all_workspaces() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["tools"], &[])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["deny.toml"])),
        ],
        vec![("tools/helper/deny.toml", build_fixture_deny_toml("service"))],
    );

    let facts = collected_facts(&tree);
    let input = forbidden_input(&facts, "tools/helper/deny.toml");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "deny config at forbidden location",
            "`tools/helper/deny.toml` (deny.toml) is at `tools/helper` which is not an allowed deny policy root.",
            "tools/helper/deny.toml",
            false,
        )],
    );
}
