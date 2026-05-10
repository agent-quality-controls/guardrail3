//! Assertion helper that proves the apparch hook contract matches the family policy.

use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Asserts the runtime-exposed hook contract equals the apparch family's expected policy.
///
/// # Panics
/// Panics when the runtime contract diverges from the expected `G3HookRequirement`.
pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_apparch_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-apparch/hook-contract".to_owned(),
            owner_family: "apparch".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
                G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::G3RsValidatePath,],
            critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned()),],
        }],
        "apparch hook contract diverged from expected policy"
    );
}
