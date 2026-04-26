use std::collections::BTreeSet;

use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-28";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let G3TsAstroPolicySurfaceState::Parsed { snapshot: policy } = &contract.astro_policy
        else {
            continue;
        };

        if contract.content_adapter_source_paths.is_empty() {
            continue;
        }

        let astro_content_sources: BTreeSet<&str> = contract
            .content_adapter_astro_content_source_paths
            .iter()
            .map(String::as_str)
            .collect();
        let adapter_sources: Vec<&str> = contract
            .content_adapter_source_paths
            .iter()
            .map(String::as_str)
            .collect();
        let missing: Vec<&str> = adapter_sources
            .iter()
            .copied()
            .filter(|path| !astro_content_sources.contains(path))
            .collect();

        if missing.is_empty() {
            results.push(crate::support::info(
                ID,
                "Astro content adapter sources import Astro content collections",
                format!(
                    "`{}` resolves `content_adapter` to adapter source files that import `astro:content` at runtime: {}.",
                    policy.rel_path,
                    format_paths(&adapter_sources)
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro content adapter source does not use Astro content collections",
            format!(
                "`{}` resolves `content_adapter` to source files that do not import `astro:content` at runtime: {}. Move non-adapter helpers outside `content_adapter`, or make each adapter source read validated Astro content through a runtime import such as `import {{ getEntry }} from \"astro:content\"`. Type-only imports do not satisfy this rule.",
                policy.rel_path,
                format_paths(&missing)
            ),
            Some(&policy.rel_path),
        ));
    }
}

fn format_paths(paths: &[&str]) -> String {
    paths
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
