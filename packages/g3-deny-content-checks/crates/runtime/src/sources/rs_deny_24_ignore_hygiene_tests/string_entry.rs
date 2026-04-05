use g3_deny_content_checks_assertions::rs_deny_24_ignore_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn simple_string_entry_errors() {
    let results = run_check(
        r#"
[advisories]
ignore = ["RUSTSEC-2024-0001"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisory ignore must use table form",
                "`deny.toml` has `[advisories].ignore` string entry `RUSTSEC-2024-0001`; use table form with a `reason`.",
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
