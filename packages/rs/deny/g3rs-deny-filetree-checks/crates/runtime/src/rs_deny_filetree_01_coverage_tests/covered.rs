use g3rs_deny_filetree_checks_assertions::rs_deny_filetree_01_coverage as assertions;

use crate::test_support::input;

#[test]
fn inventories_selected_deny_config_when_it_is_usable() {
    let input = input(Some("deny.toml"), vec!["deny.toml"], Vec::new());
    let mut results = Vec::new();

    crate::rs_deny_filetree_01_coverage::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
}
