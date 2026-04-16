use g3rs_deny_config_checks_assertions::bans::rs_deny_config_07_highlight_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_highlight_differs_from_baseline() {
    let results = run_check(
        r#"
[bans]
highlight = "simplest"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "highlight differs from baseline",
            "`deny.toml` sets `[bans].highlight = simplest`.",
            "deny.toml",
            true,
        )],
    );
}
