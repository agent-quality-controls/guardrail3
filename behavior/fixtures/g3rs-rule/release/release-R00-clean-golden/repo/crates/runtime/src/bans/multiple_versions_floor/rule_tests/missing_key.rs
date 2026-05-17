use g3rs_deny_config_checks_assertions::bans::multiple_versions_floor::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_multiple_versions_key_missing() {
    let results = run_check(
        r#"
[bans]
wildcards = "allow"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "multiple-versions missing",
            "`deny.toml` does not set `[bans].multiple-versions`.",
            "deny.toml",
            false,
        )],
    );
}
