use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookRequirement,
    G3TsHookTriggerPattern,
};

#[must_use]
pub fn hook_contract() -> Vec<G3TsHookRequirement> {
    vec![G3TsHookRequirement::new(
        "g3ts-astro-mdx/hook-contract".to_owned(),
        "astro-mdx".to_owned(),
        vec![
            G3TsHookTriggerPattern::Glob("**/*.mdx".to_owned()),
            G3TsHookTriggerPattern::Glob("src/mdx-components.*".to_owned()),
        ],
        vec![G3TsHookCommandRequirement::AppValidateScript],
        vec![G3TsHookCriticalCommand::Binary("pnpm".to_owned())],
    )]
}
