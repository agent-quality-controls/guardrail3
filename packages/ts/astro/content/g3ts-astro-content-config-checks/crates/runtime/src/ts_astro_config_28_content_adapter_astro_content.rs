use g3ts_astro_content_types::G3TsAstroContentAdapterSourceInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONTENT-CONFIG-28";

pub(crate) fn check(
    contract: &G3TsAstroContentAdapterSourceInput,
    results: &mut Vec<G3CheckResult>,
) {
    if contract.imports_astro_content {
        results.push(crate::support::info(
            ID,
            "Astro content adapter source imports Astro content collections",
            format!(
                "`{}` resolves an adapter source that imports `astro:content` at runtime: `{}`.",
                contract.policy_rel_path, contract.source_rel_path
            ),
            &contract.policy_rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro content adapter source does not use Astro content collections",
            format!(
                "`{}` resolves an adapter source that does not import `astro:content` at runtime: `{}`. Move non-adapter helpers outside `[ts.astro.content].adapters`, or make the adapter source read validated Astro content through a runtime import such as `import {{ getEntry }} from \"astro:content\"`. Type-only imports do not satisfy this rule.",
                contract.policy_rel_path,
                contract.source_rel_path
            ),
            Some(&contract.policy_rel_path),
        ));
}
