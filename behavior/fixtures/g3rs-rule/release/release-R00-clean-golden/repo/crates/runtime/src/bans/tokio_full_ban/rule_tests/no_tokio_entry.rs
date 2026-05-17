use g3rs_deny_config_checks_assertions::bans::tokio_full_ban::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_no_tokio_feature_entry() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"

[[bans.features]]
name = "serde"
deny = ["derive"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "tokio full feature not banned",
            "`deny.toml` must ban `tokio` feature `full` under `[[bans.features]]`.",
            "deny.toml",
            false,
        )],
    );
}
