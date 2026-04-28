use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

pub fn assert_contract_matches_expected_policy() {
    assert_eq!(
        g3rs_deps_hook_contract_runtime::hook_contract(),
        vec![G3HookRequirement {
            id: "g3rs-deps/hook-contract".to_owned(),
            owner_family: "deps".to_owned(),
            trigger_patterns: vec![
                G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
                G3HookTriggerPattern::ExactPath("Cargo.lock".to_owned()),
            ],
            required_commands: vec![
                G3HookCommandRequirement::CargoMachete,
                G3HookCommandRequirement::CargoDupes,
                G3HookCommandRequirement::CargoDupesExcludeTests,
            ],
            critical_commands: vec![
                G3HookCriticalCommand::CargoSubcommand("machete".to_owned()),
                G3HookCriticalCommand::Binary("cargo-machete".to_owned()),
                G3HookCriticalCommand::CargoSubcommand("dupes".to_owned()),
                G3HookCriticalCommand::Binary("cargo-dupes".to_owned()),
            ],
        }]
    );
}
