use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
    vec![G3HookRequirement {
        id: "g3rs-deny/hook-contract".to_owned(),
        owner_family: "deny".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::ExactPath("deny.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.lock".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::CargoDenyCheck],
        critical_commands: vec![
            G3HookCriticalCommand::CargoSubcommand("deny".to_owned()),
            G3HookCriticalCommand::Binary("cargo-deny".to_owned()),
        ],
    }]
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
