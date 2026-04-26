use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-02";
const DEPENDENCY_NAME: &str = "@astrojs/check";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        let has_check_package = crate::support::package_has_dependency(contract, DEPENDENCY_NAME);
        let safely_runs_astro_check = crate::support::package_safely_runs_astro_check(contract);

        if has_check_package && safely_runs_astro_check {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro check present",
                    format!("`{rel_path}` installs `{DEPENDENCY_NAME}` and safely invokes `astro check` in the app script surface."),
                    rel_path,
                ));
            }
            continue;
        }

        let missing = if !has_check_package && !safely_runs_astro_check {
            format!("`{DEPENDENCY_NAME}` is missing and no app script safely invokes `astro check`")
        } else if !has_check_package {
            format!("`{DEPENDENCY_NAME}` is missing")
        } else {
            "no app script safely invokes `astro check`".to_owned()
        };
        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` violates the Astro typecheck contract: {missing}. Install `{DEPENDENCY_NAME}` and add a script that safely runs `astro check`. Text like `echo astro check`, `astro check || true`, or unsupported shell syntax does not satisfy this rule."
            ),
            None => format!(
                "The Astro check contract could not be verified because `package.json` was not available. Restore the app package manifest, install `{DEPENDENCY_NAME}`, and add a script that safely runs `astro check`."
            ),
        };
        results.push(crate::support::error(
            ID,
            "Astro app typecheck contract is missing",
            message,
            rel_path,
        ));
    }
}
