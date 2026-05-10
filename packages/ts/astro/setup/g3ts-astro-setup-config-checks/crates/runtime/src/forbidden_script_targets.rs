use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/forbidden-script-targets";
/// Static rule data.
const FORBIDDEN_SCRIPT_TARGETS: [&str; 4] = [
    "g3ts-astro-sitemap-checks",
    "g3ts-astro-robots-checks",
    "g3ts-astro-llms-checks",
    "g3ts-astro-llms",
];

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(package) = crate::support::parsed_package(&contract.package) else {
        return;
    };

    let forbidden_targets = package
        .script_all_tool_invocations
        .iter()
        .filter(|invocation| FORBIDDEN_SCRIPT_TARGETS.contains(&invocation.executable.as_str()))
        .map(|invocation| {
            format!(
                "`{}` in script `{}`",
                invocation.executable, invocation.script_name
            )
        })
        .collect::<Vec<_>>();

    if forbidden_targets.is_empty() {
        results.push(crate::support::info(
            ID,
            "Astro package scripts do not call removed checker CLIs",
            format!(
                "`{}` does not invoke removed Astro checker CLI packages.",
                package.rel_path
            ),
            &package.rel_path,
        ));
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro package scripts call removed checker CLIs",
        format!(
            "`{}` invokes removed Astro checker CLI packages: {}. Use the approved Astro integrations instead; G3TS no longer accepts checker CLI script targets.",
            package.rel_path,
            forbidden_targets.join(", ")
        ),
        Some(&package.rel_path),
    ));
}
