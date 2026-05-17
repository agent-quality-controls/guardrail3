use g3rs_deny_config_checks_assertions::sources::ignore_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn weak_reason_errors() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "todo" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisory ignore reason too weak",
                "`deny.toml` ignores `RUSTSEC-2024-0001` with a weak `reason`: reason must not be a placeholder.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (0 documented, 0 missing reasons, 1 weak reasons).",
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
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "not needed" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisory ignore reason too weak",
                "`deny.toml` ignores `RUSTSEC-2024-0001` with a weak `reason`: reason must be at least 12 characters; found 10.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (0 documented, 0 missing reasons, 1 weak reasons).",
                "deny.toml",
                false,
            ),
        ],
    );
}
