use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPolicySnapshot};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-23";
const PROFILE: &str = "strict-local-content";
const REQUIRED_FORBIDDEN_STATE: [&str; 3] = [".next/**", ".velite/**", ".contentlayer/**"];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::astro_policy_rel_path(contract);
        if crate::support::parsed_astro_policy(contract).is_some_and(policy_is_strict) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro strict content policy is configured",
                    format!("`{rel_path}` sets `[ts.astro] profile = \"{PROFILE}\"`, declares non-empty `content_routes`, `content_root`, and `content_adapter`, and forbids `.next/**`, `.velite/**`, and `.contentlayer/**` generated state."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(crate::support::error(
            ID,
            "Astro strict content policy is missing or incomplete",
            format!(
                "`{}` must define `[ts.astro]` with `profile = \"{PROFILE}\"`, non-empty `content_routes`, `content_root`, `content_adapter`, and `forbidden_state = [\".next/**\", \".velite/**\", \".contentlayer/**\"]`. These are the only app-level Astro policy fields G3TS reads; old `*_globs` route-class fields are not supported.",
                rel_path.unwrap_or("guardrail3-rs.toml")
            ),
            rel_path,
        ));
    }
}

fn policy_is_strict(policy: &G3TsAstroPolicySnapshot) -> bool {
    policy.profile.as_deref() == Some(PROFILE)
        && !policy.content_routes.is_empty()
        && non_empty_optional_string(&policy.content_root)
        && non_empty_optional_string(&policy.content_adapter)
        && REQUIRED_FORBIDDEN_STATE
            .iter()
            .all(|required| policy.forbidden_state.iter().any(|value| value == required))
}

fn non_empty_optional_string(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}
