use g3rs_deny_config_checks_assertions::sources::skip_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn malformed_entry_errors() {
    let results = run_check(
        r#"
[bans]
skip = [{ reason = "valid reason text here" }]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "malformed skip entry",
                "`deny.toml` has `[bans.skip]` entry without a valid crate identifier.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "skip entry count",
                "`deny.toml` has 1 skip entries (0 documented, 1 missing reasons, 0 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
