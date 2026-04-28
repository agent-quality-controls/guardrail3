use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
    vec![G3HookRequirement {
        id: "g3rs-code/hook-contract".to_owned(),
        owner_family: "code".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::Glob("**/*.rs".to_owned()),
            G3HookTriggerPattern::ExactPath("guardrail3-rs.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("clippy.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("deny.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("rustfmt.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("rust-toolchain.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::G3RsValidatePath],
        critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned())],
    }]
}
