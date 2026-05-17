use g3rs_deny_config_checks_assertions::bans::duplicate_entries::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_duplicate_advisory_ignore_entries() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "First entry" },
    { id = "RUSTSEC-2024-0001", reason = "Second entry" },
]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "duplicate advisory ignore entry",
            "`deny.toml` has duplicate advisory ignore `RUSTSEC-2024-0001`.",
            "deny.toml",
            false,
        )],
    );
}
