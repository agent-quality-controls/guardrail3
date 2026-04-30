use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookRequirement,
    G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement::new(
        "g3ts-astro-setup/hook-contract".to_owned(),
        "astro-setup".to_owned(),
        vec![
            G3TsHookTriggerPattern::Glob("astro.config.*".to_owned()),
            G3TsHookTriggerPattern::Glob("package.json".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.astro".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.ts".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.tsx".to_owned()),
        ],
        vec![G3TsHookCommandRequirement::AppValidateScript],
        vec![
            G3TsHookCriticalCommand::Binary("g3ts".to_owned()),
            G3TsHookCriticalCommand::Binary("pnpm".to_owned()),
        ],
    )]
}
