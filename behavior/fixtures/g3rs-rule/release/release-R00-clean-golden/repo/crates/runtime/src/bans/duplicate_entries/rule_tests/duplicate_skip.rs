use g3rs_deny_config_checks_assertions::bans::duplicate_entries::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_duplicate_skip_entries() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0", reason = "First entry" },
    { name = "regex", version = "1.0.0", reason = "Second entry" },
]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "duplicate skip entry",
            "`deny.toml` has duplicate skip entry `regex@1.0.0`.",
            "deny.toml",
            false,
        )],
    );
}
