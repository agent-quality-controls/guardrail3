use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

use crate::hook_contract;

#[test]
fn topology_contract_matches_expected_policy() {
    assert_eq!(
        hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-topology/hook-contract".to_owned(),
            owner_family: "topology".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            ],
            required_commands: vec![G3HookCommandRequirement::G3RsValidatePath,],
            critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned()),],
        }]
    );
}
