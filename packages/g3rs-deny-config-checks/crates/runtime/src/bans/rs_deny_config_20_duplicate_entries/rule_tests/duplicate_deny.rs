use g3rs_deny_config_checks_assertions::rs_deny_config_20_duplicate_entries as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_duplicate_deny_entries() {
    let results = run_check(
        r#"
[bans]
deny = [
    "some-crate",
    { name = "some-crate" },
]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "duplicate deny entry",
            "`deny.toml` has duplicate deny entry `some-crate`.",
            "deny.toml",
            false,
        )],
    );
}
