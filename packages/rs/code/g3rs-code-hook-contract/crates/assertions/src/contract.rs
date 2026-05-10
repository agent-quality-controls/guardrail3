use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Assert the runtime hook contract matches the expected policy.
///
/// # Panics
///
/// Panics on any contract drift.
pub fn assert_contract_matches_expected_policy() {
    let actual = g3rs_code_hook_contract_runtime::hook_contract();
    let expected = vec![G3HookRequirement {
        id: "g3rs-code/hook-contract".to_owned(),
        owner_family: "code".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
            G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("clippy.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("deny.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("rustfmt.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("rust-toolchain.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::G3RsValidatePath],
        critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned())],
    }];
    assert_eq!(actual, expected, "code hook contract drift: {actual:#?}");
}
