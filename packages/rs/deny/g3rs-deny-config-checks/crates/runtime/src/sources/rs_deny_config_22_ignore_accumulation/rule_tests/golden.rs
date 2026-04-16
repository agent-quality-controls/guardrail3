use g3rs_deny_config_checks_assertions::sources::rs_deny_config_22_ignore_accumulation::rule as assertions;

use super::helpers::run_check;

#[test]
fn five_or_fewer_entries_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "Not applicable to our usage pattern" },
    { id = "RUSTSEC-2024-0002", reason = "Waiting for upstream fix release cycle" },
    { id = "RUSTSEC-2024-0003", reason = "Only affects Windows and we deploy on Linux" },
    { id = "RUSTSEC-2024-0004", reason = "Mitigated by input validation layer" },
    { id = "RUSTSEC-2024-0005", reason = "Advisory withdrawn by maintainer team" },
]
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn empty_ignore_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
ignore = []
"#,
    );
    assertions::assert_no_findings(&results);
}
