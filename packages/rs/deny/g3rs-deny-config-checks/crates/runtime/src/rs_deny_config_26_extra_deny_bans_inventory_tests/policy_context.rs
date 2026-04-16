use g3rs_deny_config_checks_assertions::rs_deny_config_26_extra_deny_bans_inventory as assertions;
use g3rs_deny_types::G3RsDenyRustPolicyState;

use test_support::{run, run_with_rust_policy};

#[test]
fn stays_quiet_when_policy_context_is_invalid() {
    let results = run(
        r#"
[bans]
deny = ["custom-crate"]
"#,
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        false,
        crate::rs_deny_config_26_extra_deny_bans_inventory::check,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn stays_quiet_when_rust_policy_is_unreadable() {
    let results = run_with_rust_policy(
        r#"
[bans]
deny = ["custom-crate"]
"#,
        G3RsDenyRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "file is not readable".to_owned(),
        },
        crate::rs_deny_config_26_extra_deny_bans_inventory::check,
    );

    assertions::assert_no_findings(&results);
}
