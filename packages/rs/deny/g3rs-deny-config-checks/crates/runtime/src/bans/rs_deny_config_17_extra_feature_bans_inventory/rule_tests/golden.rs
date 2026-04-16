use g3rs_deny_config_checks_assertions::bans::rs_deny_config_17_extra_feature_bans_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_only_tokio_entry() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros"]
"#,
    );

    assertions::assert_no_findings(&results);
}
