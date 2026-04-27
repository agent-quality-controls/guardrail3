use g3rs_toolchain_config_checks_assertions::msrv_consistency::rule as assertions;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_channel_is_prerelease() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0-dev"
"#,
        r#"
[package]
name = "fixture"
version = "0.1.0"
edition = "2024"
rust-version = "1.84"
"#,
    );

    assertions::assert_findings(&results, &[]);
}
