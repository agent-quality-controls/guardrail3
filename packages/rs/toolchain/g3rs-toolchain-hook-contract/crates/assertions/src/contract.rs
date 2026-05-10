use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Asserts that the toolchain family's hook contract matches the expected policy.
///
/// # Panics
/// Panics when the runtime-provided hook contract does not match the expected
/// trigger patterns, required commands, or critical commands.
pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_toolchain_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-toolchain/hook-contract".to_owned(),
            owner_family: "toolchain".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::ExactPath("rust-toolchain.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::G3RsValidatePath,],
            critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned()),],
        }],
        "toolchain hook contract must match the expected policy snapshot",
    );
}
