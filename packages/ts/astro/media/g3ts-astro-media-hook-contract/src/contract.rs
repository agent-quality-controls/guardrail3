use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookRequirement, G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement::new(
        "g3ts-astro-media/hook-contract".to_owned(),
        "astro-media".to_owned(),
        vec![
            G3TsHookTriggerPattern::Glob("eslint.config.*".to_owned()),
            G3TsHookTriggerPattern::Glob("guardrail3-ts.toml".to_owned()),
            G3TsHookTriggerPattern::Glob("astro.config.*".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.astro".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.ts".to_owned()),
            G3TsHookTriggerPattern::Glob("src/**/*.tsx".to_owned()),
            G3TsHookTriggerPattern::Glob("content/**/*.mdx".to_owned()),
        ],
        vec![
            G3TsHookCommandRequirement::G3TsValidatePath,
            G3TsHookCommandRequirement::AppValidateScript,
        ],
        Vec::new(),
    )]
}
