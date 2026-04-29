use g3ts_astro_media_types::G3TsAstroMediaIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-media/media-build-validation-runs";

pub(crate) fn check(
    contract: &G3TsAstroMediaIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    if crate::support::package_safely_runs_astro_build(&contract.package) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro media build validation runs",
                format!("`{rel_path}` has a fail-closed `validate` script that runs `astro build`, so media asset integration failures break validation."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        ID,
        "Astro media build validation does not run",
        format!(
            "`{}` must define a fail-closed `validate` script that runs `astro build`. Media asset checks run in the Astro build lifecycle, not inside G3TS.",
            rel_path.unwrap_or("package.json")
        ),
        rel_path,
    ));
}
