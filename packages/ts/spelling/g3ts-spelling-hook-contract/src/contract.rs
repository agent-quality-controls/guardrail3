use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookRequirement, G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement::new(
        "g3ts-spelling/hook-contract".to_owned(),
        "spelling".to_owned(),
        vec![
            G3TsHookTriggerPattern::Glob("package.json".to_owned()),
            G3TsHookTriggerPattern::Glob(".syncpackrc".to_owned()),
            G3TsHookTriggerPattern::Glob("cspell.json".to_owned()),
            G3TsHookTriggerPattern::Glob("cspell.config.json".to_owned()),
            G3TsHookTriggerPattern::Glob(".cspell.json".to_owned()),
            G3TsHookTriggerPattern::Glob("cspell.yaml".to_owned()),
            G3TsHookTriggerPattern::Glob("cspell.yml".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.ts".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.tsx".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.astro".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.md".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.mdx".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.json".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.yml".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.yaml".to_owned()),
        ],
        vec![
            G3TsHookCommandRequirement::G3TsValidatePath,
            G3TsHookCommandRequirement::AppValidateScript,
        ],
        Vec::new(),
    )]
}
