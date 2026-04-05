use g3_deny_content_checks_assertions::rs_deny_23_skip_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn table_entry_without_reason_errors() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "skip entry missing reason",
                "`deny.toml` skips `regex` without a `reason`.",
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
