use g3_deny_content_checks_assertions::rs_deny_13_wildcards_inventory as assertions;

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
