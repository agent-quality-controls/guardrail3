use g3_deny_content_checks_assertions::rs_deny_24_ignore_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn documented_ignore_entry_warns() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "Not applicable because we never expose the affected API surface to untrusted input" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "advisory ignore entry",
                "`deny.toml` has documented advisory ignore `RUSTSEC-2024-0001`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (1 documented, 0 missing reasons, 0 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
