use g3rs_deny_config_checks_assertions::bans::rs_deny_config_06_multiple_versions_floor::rule as assertions;

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
