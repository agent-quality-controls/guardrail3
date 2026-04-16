use g3rs_deny_config_checks_assertions::sources::rs_deny_config_19_ignore_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn malformed_entry_errors() {
    let results = run_check(
        r#"
[advisories]
ignore = [{ reason = "valid reason text here" }]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "malformed advisory ignore entry",
                "`deny.toml` has an `[advisories].ignore` entry without a valid advisory id or package selector.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (0 documented, 1 missing reasons, 0 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
