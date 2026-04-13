use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use crate::test_support::run;

#[test]
fn skips_allow_list_findings_when_policy_context_is_invalid() {
    let results = run(
        r#"
[bans]
allow = ["demo"]
"#,
        Some("service"),
        false,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_no_findings(&results);
}
