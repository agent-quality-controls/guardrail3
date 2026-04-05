use g3_deny_content_checks_assertions::rs_deny_10_multiple_versions_floor as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_multiple_versions_is_warn() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "warn"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "multiple-versions weaker than baseline",
            "`deny.toml` sets `[bans].multiple-versions = \"warn\"`.",
            "deny.toml",
            false,
        )],
    );
}
