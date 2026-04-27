use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;

const CONTENT_ID: &str = "TS-ASTRO-CONTENT-CONFIG-23";
const PROFILE: &str = "strict-static-content";

pub(crate) fn check_content(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::astro_policy_rel_path(contract);
        if crate::support::parsed_astro_policy(contract).is_some_and(policy_content_is_strict) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    CONTENT_ID,
                    "Astro strict content policy is configured",
                    format!("`{rel_path}` sets `[ts.astro] profile = \"{PROFILE}\"`, declares non-empty `[ts.astro.routes].content`, `[ts.astro.content].root`, and `[ts.astro.content].adapters`."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(crate::support::error(
            CONTENT_ID,
            "Astro strict content policy is missing or incomplete",
            format!(
                "`{}` must define `[ts.astro] profile = \"{PROFILE}\"`, non-empty `[ts.astro.routes].content`, `[ts.astro.content].root`, and `[ts.astro.content].adapters`. These are app-level Astro content capability fields G3TS reads; old flat `[ts.astro]` fields and old `*_globs` route-class fields are not supported.",
                rel_path.unwrap_or("guardrail3-ts.toml")
            ),
            rel_path,
        ));
    }
}

fn policy_content_is_strict(policy: &G3TsAstroPolicySnapshot) -> bool {
    policy.profile.as_deref() == Some(PROFILE)
        && !policy.content_routes.is_empty()
        && non_empty_optional_string(&policy.content_root)
        && !policy.content_adapters.is_empty()
}

fn non_empty_optional_string(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}
