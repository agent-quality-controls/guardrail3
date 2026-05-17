use g3rs_deny_config_checks_assertions::bans::duplicate_entries::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_no_duplicates() {
    let results = run_check(
        r#"
[bans]
deny = [
    "crate-a",
    { name = "crate-b" },
]
skip = [
    { name = "regex", version = "1.0.0", reason = "Pinned for compatibility" },
]

[[bans.features]]
name = "tokio"
deny = ["full"]

[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "Not applicable to our usage" },
]
"#,
    );

    assertions::assert_no_findings(&results);
}
