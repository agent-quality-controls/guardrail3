use g3_deny_content_checks_assertions::rs_deny_21_tokio_full_ban as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_tokio_entry_does_not_deny_full() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"

[[bans.features]]
name = "tokio"
deny = ["test-util"]
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]
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
