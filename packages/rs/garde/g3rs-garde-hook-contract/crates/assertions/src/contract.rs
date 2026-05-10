use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Assert that the runtime hook contract matches the expected garde-family policy.
///
/// # Panics
/// Panics if the runtime contract differs from the expected requirement set.
pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_garde_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-garde/hook-contract".to_owned(),
            owner_family: "garde".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
                G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::G3RsValidatePath,],
            critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned()),],
        }],
        "garde hook contract diverged from expected policy",
    );
}
