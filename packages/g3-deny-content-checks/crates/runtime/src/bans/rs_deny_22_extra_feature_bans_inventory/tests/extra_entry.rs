use g3_deny_content_checks_assertions::rs_deny_22_extra_feature_bans_inventory as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_extra_feature_ban_entry_exists() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"

[[bans.features]]
name = "tokio"
deny = ["full"]

[[bans.features]]
name = "serde"
deny = ["derive"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "extra feature ban",
            "`deny.toml` has extra feature-ban entry for `serde`.",
            "deny.toml",
            true,
        )],
    );
}
