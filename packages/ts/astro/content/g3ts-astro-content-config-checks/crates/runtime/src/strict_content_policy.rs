use g3ts_astro_content_types::{
    G3TsAstroContentIntegrationContractInput, G3TsAstroContentPolicySnapshot,
};
use guardrail3_check_types::G3CheckResult;

/// Internal constant `CONTENT_ID`.
const CONTENT_ID: &str = "g3ts-astro-content/strict-content-policy";
/// Internal constant `PROFILE`.
const PROFILE: &str = "strict-static-content";

/// Internal function `check_content`.
pub(crate) fn check_content(
    contract: &G3TsAstroContentIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::content_policy_rel_path(&contract.astro_policy);
    if crate::support::parsed_content_policy(&contract.astro_policy)
        .is_some_and(policy_content_is_strict)
    {
        results.push(crate::support::info(
                CONTENT_ID,
                "Astro strict content policy is configured",
                format!("`{rel_path}` sets `[ts.astro] profile = \"{PROFILE}\"`, declares non-empty `[ts.astro.routes].content`, `[ts.astro.content].root`, and `[ts.astro.content].adapters`."),
                rel_path,
            ));
        return;
    }

    results.push(crate::support::error(
            CONTENT_ID,
            "Astro strict content policy is missing or incomplete",
            format!(
                "`{rel_path}` must define `[ts.astro] profile = \"{PROFILE}\"`, non-empty `[ts.astro.routes].content`, `[ts.astro.content].root`, and `[ts.astro.content].adapters`. These are app-level Astro content capability fields G3TS reads; old flat `[ts.astro]` fields and old `*_globs` route-class fields are not supported."
            ),
            Some(rel_path),
        ));
}

/// Internal function `policy_content_is_strict`.
fn policy_content_is_strict(policy: &G3TsAstroContentPolicySnapshot) -> bool {
    policy.profile.as_deref() == Some(PROFILE)
        && !policy.content_routes.is_empty()
        && non_empty_optional_string(policy.content_root.as_deref())
        && !policy.content_adapters.is_empty()
}

/// Returns `true` when the optional string is present and non-empty.
fn non_empty_optional_string(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.is_empty())
}
