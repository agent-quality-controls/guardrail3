use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Asserts that the topology family's hook contract matches the expected policy.
///
/// # Panics
/// Panics when the runtime-provided hook contract does not match the expected
/// trigger patterns, required commands, or critical commands.
pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_topology_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-topology/hook-contract".to_owned(),
            owner_family: "topology".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::G3RsValidatePath,],
            critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned()),],
        }],
        "topology hook contract must match the expected policy snapshot",
    );
}
