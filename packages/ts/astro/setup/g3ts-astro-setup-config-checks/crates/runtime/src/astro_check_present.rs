use g3ts_astro_setup_types::G3TsAstroSetupIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-astro-setup/astro-check-present";
/// Required npm dependency name.
const DEPENDENCY_NAME: &str = "@astrojs/check";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(
    contract: &G3TsAstroSetupIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    let has_check_package =
        crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME);
    let safely_runs_astro_check =
        crate::support::package_safely_runs_astro_check(&contract.package);

    if has_check_package && safely_runs_astro_check {
        results.push(crate::support::info(
                ID,
                "astro check present",
                format!("`{rel_path}` installs `{DEPENDENCY_NAME}` and safely invokes `astro check` in the app script surface."),
                rel_path,
            ));
        return;
    }

    let missing = if !has_check_package && !safely_runs_astro_check {
        format!("`{DEPENDENCY_NAME}` is missing and no app script safely invokes `astro check`")
    } else if !has_check_package {
        format!("`{DEPENDENCY_NAME}` is missing")
    } else {
        "no app script safely invokes `astro check`".to_owned()
    };
    let message = format!(
        "`{rel_path}` violates the Astro typecheck contract: {missing}. Install `{DEPENDENCY_NAME}` and add a script that safely runs `astro check`. Text like `echo astro check`, `astro check || true`, or unsupported shell syntax does not satisfy this rule."
    );
    results.push(crate::support::error(
        ID,
        "Astro app typecheck contract is missing",
        message,
        Some(rel_path),
    ));
}
