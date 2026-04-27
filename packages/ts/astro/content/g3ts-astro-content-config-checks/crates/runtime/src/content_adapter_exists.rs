use g3ts_astro_content_types::G3TsAstroContentAdapterRootInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-content/content-adapter-exists";

pub(crate) fn check(contract: &G3TsAstroContentAdapterRootInput, results: &mut Vec<G3CheckResult>) {
    if contract.source_exists {
        results.push(crate::support::info(
            ID,
            "Astro content adapter source exists",
            format!(
                "`{}` resolves configured content adapter `{}` to at least one adapter source file.",
                contract.policy_rel_path, contract.configured_adapter
            ),
            &contract.policy_rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Astro content adapter source is missing",
            format!(
                "`{}` sets `[ts.astro.content].adapters` to `{}`, but no included adapter source file exists at or below that configured adapter path. Create an app-local adapter source there; routes must use adapters instead of reading Astro content directly.",
                contract.policy_rel_path,
                contract.configured_adapter
            ),
            Some(&contract.policy_rel_path),
        ));
}
