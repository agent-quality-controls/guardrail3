use g3rs_deny_config_checks_assertions::bans::tokio_full_ban::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_tokio_allow_list_changes() {
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

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "tokio allowed features changed",
            "`deny.toml` must keep `tokio` allowed features `bytes, default, io-util, macros, net, rt-multi-thread, signal, sync, time`.",
            "deny.toml",
            false,
        )],
    );
}
