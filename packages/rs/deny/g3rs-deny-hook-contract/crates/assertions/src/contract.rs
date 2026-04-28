use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

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
        }]
    );
}
