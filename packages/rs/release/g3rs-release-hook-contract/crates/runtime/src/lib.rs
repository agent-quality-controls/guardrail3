use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3HookRequirement> {
    vec![G3HookRequirement {
        id: "g3rs-release/hook-contract".to_owned(),
        owner_family: "release".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::ExactPath("release-plz.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("cliff.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.lock".to_owned()),
            G3HookTriggerPattern::ExactPath("README.md".to_owned()),
            G3HookTriggerPattern::ExactPath("readme.md".to_owned()),
            G3HookTriggerPattern::ExactPath("Readme.md".to_owned()),
            G3HookTriggerPattern::Glob(".github/workflows/*.yml".to_owned()),
            G3HookTriggerPattern::Glob(".github/workflows/*.yaml".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::G3RsValidatePath],
        critical_commands: vec![G3HookCriticalCommand::Binary("g3rs".to_owned())],
    }]
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
