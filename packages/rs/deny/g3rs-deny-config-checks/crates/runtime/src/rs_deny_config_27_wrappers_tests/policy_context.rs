use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use crate::test_support::run;

#[test]
fn stays_quiet_when_policy_context_is_invalid() {
    let results = run(
        r#"
[bans]
deny = [{ name = "custom-crate", wrappers = ["adapter"] }]
"#,
        Some("service"),
        false,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_no_findings(&results);
}
