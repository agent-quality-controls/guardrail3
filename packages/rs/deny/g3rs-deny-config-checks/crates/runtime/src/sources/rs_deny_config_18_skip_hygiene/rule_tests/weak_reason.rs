use g3rs_deny_config_checks_assertions::sources::rs_deny_config_18_skip_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn weak_reason_errors() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0", reason = "todo" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "skip entry reason too weak",
                "`deny.toml` skips `regex` with a weak `reason`: reason must not be a placeholder.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "skip entry count",
                "`deny.toml` has 1 skip entries (0 documented, 0 missing reasons, 1 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}

#[test]
fn too_short_reason_errors() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0", reason = "short text" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "skip entry reason too weak",
                "`deny.toml` skips `regex` with a weak `reason`: reason must be at least 12 characters; found 10.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "skip entry count",
                "`deny.toml` has 1 skip entries (0 documented, 0 missing reasons, 1 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
