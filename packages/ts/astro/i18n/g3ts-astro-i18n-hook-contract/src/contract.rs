use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookRequirement,
    G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement {
        id: "g3ts-astro-i18n/hook-contract".to_owned(),
        owner_family: "astro-i18n".to_owned(),
        trigger_patterns: vec![
            G3TsHookTriggerPattern::Glob("eslint.config.*".to_owned()),
            G3TsHookTriggerPattern::Glob("guardrail3-ts.toml".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.astro".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.ts".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.tsx".to_owned()),
            G3TsHookTriggerPattern::Glob("content/**/*.mdx".to_owned()),
        ],
        required_commands: vec![
            G3TsHookCommandRequirement::G3TsValidatePath,
            G3TsHookCommandRequirement::AppValidateScript,
        ],
        critical_commands: vec![G3TsHookCriticalCommand::Binary("pnpm".to_owned())],
    }]
}
