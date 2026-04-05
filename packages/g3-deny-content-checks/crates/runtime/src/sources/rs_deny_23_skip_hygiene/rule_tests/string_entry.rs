use g3_deny_content_checks_assertions::rs_deny_23_skip_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn simple_string_entry_errors() {
    let results = run_check(
        r#"
[bans]
skip = ["windows-sys"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "skip entry must use table form",
                "`deny.toml` has `[bans.skip]` string entry `windows-sys`; use table form with a `reason`.",
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
