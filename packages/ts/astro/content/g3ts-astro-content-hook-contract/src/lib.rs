use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookRequirement,
    G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement {
        id: "g3ts-astro-content/hook-contract".to_owned(),
        owner_family: "astro-content".to_owned(),
        trigger_patterns: vec![
            G3TsHookTriggerPattern::Glob("src/content.config.*".to_owned()),
            G3TsHookTriggerPattern::Glob("content/**/*".to_owned()),
            G3TsHookTriggerPattern::Glob("src/content/**/*".to_owned()),
        ],
        required_commands: vec![G3TsHookCommandRequirement::AppValidateScript],
        critical_commands: vec![G3TsHookCriticalCommand::Binary("pnpm".to_owned())],
    }]
}
