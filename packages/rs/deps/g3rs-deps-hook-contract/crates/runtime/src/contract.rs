use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
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
}

#[cfg(test)]
#[path = "contract_tests/mod.rs"] // reason: owned sidecar tests for contract module.
mod contract_tests;
