use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
    vec![G3HookRequirement {
        id: "g3rs-fmt/hook-contract".to_owned(),
        owner_family: "fmt".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
            G3HookTriggerPattern::ExactPath("rustfmt.toml".to_owned()),
            G3HookTriggerPattern::ExactPath(".rustfmt.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::CargoFmtCheck],
        critical_commands: vec![G3HookCriticalCommand::CargoSubcommand("fmt".to_owned())],
    }]
}

#[cfg(test)]
#[path = "contract_tests/mod.rs"] // reason: owned sidecar tests for contract module.
mod contract_tests;
