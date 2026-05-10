use g3rs_deny_filetree_checks_assertions::coverage as assertions;
use test_support::input;

#[test]
fn reports_selected_deny_parse_failures_without_hiding_coverage_inventory() {
    let msg = "Failed to parse root deny config `deny.toml` for deny checks: invalid TOML.";
    let mut results = Vec::new();
    crate::coverage::check(
        &input(
            Some("deny.toml"),
            vec!["deny.toml"],
            vec![("deny.toml", msg)],
        ),
        &mut results,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error("deny input failure", msg, "deny.toml", false),
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
    let path = "guardrail3-rs.toml";
    let msg = "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: invalid policy.";
    let mut output = Vec::new();
    let case = input(Some("deny.toml"), vec!["deny.toml"], vec![(path, msg)]);
    crate::coverage::check(&case, &mut output);
    assertions::assert_findings(
        &output,
        &[
            assertions::error("deny rust policy is not parseable", msg, path, false),
            assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
