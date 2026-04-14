use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;
use g3rs_deny_types::G3RsDenyRustPolicyState;

use crate::test_support::{run, run_with_rust_policy};

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

#[test]
fn skips_allow_list_findings_when_rust_policy_is_unreadable() {
    let results = run_with_rust_policy(
        r#"
[bans]
allow = ["demo"]
"#,
        G3RsDenyRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "file is not readable".to_owned(),
        },
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_no_findings(&results);
}
