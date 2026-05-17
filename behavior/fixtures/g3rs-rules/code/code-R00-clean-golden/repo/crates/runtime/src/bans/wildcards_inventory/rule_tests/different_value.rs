use g3rs_deny_config_checks_assertions::bans::wildcards_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_wildcards_differs_from_baseline() {
    let results = run_check(
        r#"
[bans]
wildcards = "deny"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "wildcards differs from baseline",
            "`deny.toml` sets `[bans].wildcards = deny`.",
            "deny.toml",
            false,
        )],
    );
}
