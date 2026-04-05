use g3_deny_content_checks_assertions::rs_deny_21_tokio_full_ban as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_tokio_full_is_banned_correctly() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"

[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]
"#,
    );

    assertions::assert_no_findings(&results);
}
