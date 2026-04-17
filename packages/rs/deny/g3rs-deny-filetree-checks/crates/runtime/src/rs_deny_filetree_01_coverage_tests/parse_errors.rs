use g3rs_deny_filetree_checks_assertions::rs_deny_filetree_01_coverage as assertions;
use test_support::input;

#[test]
fn reports_selected_deny_parse_failures_without_hiding_coverage_inventory() {
    let input = input(
        Some("deny.toml"),
        vec!["deny.toml"],
        vec![(
            "deny.toml",
            "Failed to parse root deny config `deny.toml` for deny checks: invalid TOML.",
        )],
    );
    let mut results = Vec::new();

    crate::rs_deny_filetree_01_coverage::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "deny input failure",
                "Failed to parse root deny config `deny.toml` for deny checks: invalid TOML.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn reports_policy_context_failures_without_hiding_selected_coverage() {
    let input = input(
        Some("deny.toml"),
        vec!["deny.toml"],
        vec![(
            "guardrail3-rs.toml",
            "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: invalid policy.",
        )],
    );
    let mut results = Vec::new();

    crate::rs_deny_filetree_01_coverage::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "deny rust policy is not parseable",
                "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: invalid policy.",
                "guardrail3-rs.toml",
                false,
            ),
            assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
