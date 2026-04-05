use g3_deny_content_checks_assertions::rs_deny_21_tokio_full_ban as assertions;

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
