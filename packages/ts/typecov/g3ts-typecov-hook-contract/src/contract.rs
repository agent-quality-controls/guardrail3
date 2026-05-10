use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookRequirement, G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement::new(
        "g3ts-typecov/hook-contract".to_owned(),
        "typecov".to_owned(),
        vec![
            G3TsHookTriggerPattern::Glob("package.json".to_owned()),
            G3TsHookTriggerPattern::Glob(".syncpackrc".to_owned()),
            G3TsHookTriggerPattern::Glob("tsconfig*.json".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.ts".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.tsx".to_owned()),
            G3TsHookTriggerPattern::Glob("**/*.astro".to_owned()),
        ],
        vec![
            G3TsHookCommandRequirement::G3TsValidatePath,
            G3TsHookCommandRequirement::AppValidateScript,
            G3TsHookCommandRequirement::TypeCoverage,
        ],
        Vec::new(),
    )]
}
