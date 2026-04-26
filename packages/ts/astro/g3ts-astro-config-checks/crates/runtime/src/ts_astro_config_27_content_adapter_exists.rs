use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-27";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let G3TsAstroPolicySurfaceState::Parsed { snapshot: policy } = &contract.astro_policy
        else {
            continue;
        };
        let content_adapter = policy
            .content_adapter
            .as_deref()
            .unwrap_or("content_adapter");

        if !contract.content_adapter_source_paths.is_empty() {
            results.push(crate::support::info(
                ID,
                "Astro content adapter source exists",
                format!(
                    "`{}` resolves `content_adapter = \"{content_adapter}\"` to adapter source files: {}.",
                    policy.rel_path,
                    format_paths(&contract.content_adapter_source_paths)
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro content adapter source is missing",
            format!(
                "`{}` sets `content_adapter = \"{content_adapter}\"`, but no included adapter source file exists at that path or below it. Create app-local adapter source under that path; routes must use adapters instead of reading Astro content directly.",
                policy.rel_path
            ),
            Some(&policy.rel_path),
        ));
    }
}

fn format_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
