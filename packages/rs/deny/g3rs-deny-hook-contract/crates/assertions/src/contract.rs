//! Contract-shape assertions for the g3rs deny family.

use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Assert the runtime-exported hook contract matches the policy expected by
/// downstream consumers.
///
/// # Panics
///
/// Panics if the runtime contract drifts from the expected shape (for example
/// if a trigger pattern, required command, or critical command is added,
/// removed, or renamed without updating this assertion).
pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_deny_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-deny/hook-contract".to_owned(),
            owner_family: "deny".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::ExactPath("deny.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.lock".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::CargoDenyCheck,],
            critical_commands: vec![
                G3HookCriticalCommand::CargoSubcommand("deny".to_owned()),
                G3HookCriticalCommand::Binary("cargo-deny".to_owned()),
            ],
        }],
        "g3rs deny hook contract drifted from the expected policy shape"
    );
}
