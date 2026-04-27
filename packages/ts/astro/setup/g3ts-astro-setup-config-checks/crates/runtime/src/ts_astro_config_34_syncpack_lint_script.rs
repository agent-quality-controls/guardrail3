use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-34";

pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);

    if crate::support::package_safely_runs_syncpack_lint(&contract.package) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Astro app package lint script runs Syncpack",
                format!(
                    "`{rel_path}` has a fail-closed `lint:packages` script that invokes `syncpack lint`. Astro stack pins and forbidden dependency bans only protect the app when Syncpack is actually runnable from the app package surface."
                ),
                rel_path,
            ));
        }
        return;
    }

    let message = match rel_path {
        Some(rel_path) => format!(
            "`{rel_path}` must define a fail-closed `lint:packages` script that invokes `syncpack lint`. The script must not hide failures through `|| true`, unsupported shell syntax, or an ignored parser blocker. G3TS delegates Astro dependency floors and banned dependency policy to Syncpack, so a parseable `.syncpackrc` without a runnable package lint script is not enough."
        ),
        None => "The Astro Syncpack execution contract could not be checked because `package.json` was not available. Restore the app package manifest and add a fail-closed `lint:packages` script that invokes `syncpack lint`.".to_owned(),
    };

    results.push(crate::support::error(
        ID,
        "Astro app package lint script does not run Syncpack",
        message,
        rel_path,
    ));
}
