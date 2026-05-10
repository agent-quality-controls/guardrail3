use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/lint-script";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);

    if crate::support::package_safely_runs_eslint(&contract.package) {
        results.push(crate::support::info(
            ID,
            "Astro app lint script runs ESLint",
            format!(
                "`{rel_path}` has a fail-closed `lint` script that invokes `eslint`. Astro content, MDX, metadata, JSON-LD, and inline-copy plugin rules only protect the app when ESLint is actually runnable from the app package surface."
            ),
            rel_path,
        ));
        return;
    }

    let message = format!(
        "`{rel_path}` must define a fail-closed `lint` script that invokes `eslint`, for example `eslint --max-warnings 0 .`. The script must not hide failures through `|| true`, unsupported shell syntax, or an ignored parser blocker. Astro delegates content, MDX, metadata, JSON-LD, and inline-copy source checks to ESLint, so wiring the plugins without a runnable lint script is not enough."
    );

    results.push(crate::support::error(
        ID,
        "Astro app lint script does not run ESLint",
        message,
        Some(rel_path),
    ));
}
