use g3_deny_content_checks_assertions::rs_deny_23_skip_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn documented_skip_entry_warns() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0", reason = "Pinned for compat with tree-sitter which requires this exact version" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "skip entry",
                "`deny.toml` has documented skip entry `regex`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "skip entry count",
                "`deny.toml` has 1 skip entries (1 documented, 0 missing reasons, 0 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
