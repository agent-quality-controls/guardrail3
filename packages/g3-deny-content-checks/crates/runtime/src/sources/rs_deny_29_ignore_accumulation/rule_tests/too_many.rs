use g3_deny_content_checks_assertions::rs_deny_29_ignore_accumulation as assertions;

use super::helpers::run_check;

#[test]
fn six_entries_warns() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "Not applicable to our usage pattern" },
    { id = "RUSTSEC-2024-0002", reason = "Waiting for upstream fix release cycle" },
    { id = "RUSTSEC-2024-0003", reason = "Only affects Windows and we deploy on Linux" },
    { id = "RUSTSEC-2024-0004", reason = "Mitigated by input validation layer" },
    { id = "RUSTSEC-2024-0005", reason = "Advisory withdrawn by maintainer team" },
    { id = "RUSTSEC-2024-0006", reason = "Covered by our custom security policy" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5).",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn many_entries_warns_with_correct_count() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "Not applicable to our usage pattern" },
    { id = "RUSTSEC-2024-0002", reason = "Waiting for upstream fix release cycle" },
    { id = "RUSTSEC-2024-0003", reason = "Only affects Windows and we deploy on Linux" },
    { id = "RUSTSEC-2024-0004", reason = "Mitigated by input validation layer" },
    { id = "RUSTSEC-2024-0005", reason = "Advisory withdrawn by maintainer team" },
    { id = "RUSTSEC-2024-0006", reason = "Covered by our custom security policy" },
    { id = "RUSTSEC-2024-0007", reason = "Not exploitable in our build configuration" },
    { id = "RUSTSEC-2024-0008", reason = "Awaiting patch from upstream maintainer" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`deny.toml` has 8 `[advisories].ignore` entries (threshold: 5).",
            "deny.toml",
            false,
        )],
    );
}
