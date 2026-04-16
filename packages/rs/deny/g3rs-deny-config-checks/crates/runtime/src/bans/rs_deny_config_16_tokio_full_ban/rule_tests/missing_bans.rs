use g3rs_deny_config_checks_assertions::bans::rs_deny_config_16_tokio_full_ban::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_bans_section_missing() {
    let results = run_check("");

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
