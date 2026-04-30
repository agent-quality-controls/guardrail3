use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
    vec![G3HookRequirement {
        id: "g3rs-arch/hook-contract".to_owned(),
        owner_family: "arch".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
            G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::G3RsValidatePath],
        critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned())],
    }]
}

#[cfg(test)]
#[path = "contract_tests/mod.rs"] // reason: owned sidecar tests for contract module.
mod contract_tests;
