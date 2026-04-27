use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-27";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let G3TsAstroPolicySurfaceState::Parsed { snapshot: policy } = &contract.astro_policy
        else {
            continue;
        };
        if policy.content_adapters.is_empty() {
            continue;
        }

        let missing_adapters = missing_adapter_roots(
            &policy.content_adapters,
            &contract.approved_surface_sources.content_adapter,
        );

        if missing_adapters.is_empty() {
            results.push(crate::support::info(
                ID,
                "Astro content adapter source exists",
                format!(
                    "`{}` resolves `[ts.astro.content].adapters = [{}]` to adapter source files: {}.",
                    policy.rel_path,
                    format_quoted_paths(&policy.content_adapters),
                    format_paths(&contract.approved_surface_sources.content_adapter)
                ),
                &policy.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro content adapter source is missing",
            format!(
                "`{}` sets `[ts.astro.content].adapters = [{}], but no included adapter source file exists at or below these configured adapter paths: {}. Create app-local adapter source under every configured adapter path; routes must use adapters instead of reading Astro content directly.",
                policy.rel_path,
                format_quoted_paths(&policy.content_adapters),
                format_paths(&missing_adapters)
            ),
            Some(&policy.rel_path),
        ));
    }
}

fn missing_adapter_roots(configured_adapters: &[String], source_paths: &[String]) -> Vec<String> {
    configured_adapters
        .iter()
        .filter(|adapter| !adapter_has_source(adapter, source_paths))
        .cloned()
        .collect()
}

fn adapter_has_source(adapter: &str, source_paths: &[String]) -> bool {
    let adapter = adapter.trim_end_matches('/');
    let adapter_prefix = format!("{adapter}/");
    source_paths
        .iter()
        .any(|path| path == adapter || path.starts_with(&adapter_prefix))
}

fn format_quoted_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("\"{path}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_paths(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
