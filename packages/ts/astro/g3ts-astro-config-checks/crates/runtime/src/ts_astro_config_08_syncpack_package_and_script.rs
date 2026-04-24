use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-08";
const DEPENDENCY_NAME: &str = "syncpack";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        check_package(contract, results);
    }
}

fn check_package(
    contract: &g3ts_astro_types::G3TsAstroIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::package_rel_path(contract);
    let has_dependency = crate::support::package_has_dependency(contract, DEPENDENCY_NAME);
    let has_script = crate::support::package_safely_runs_syncpack_lint(contract);
    let invokes_script = crate::support::package_invokes_tool(contract, "syncpack", "lint");

    if has_dependency && has_script {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                ID,
                "Syncpack package policy validator is installed and wired",
                format!("`{rel_path}` includes `syncpack` and invokes `syncpack lint`."),
                rel_path,
            ));
        }
        return;
    }

    let message = match (rel_path, has_dependency, has_script, invokes_script) {
        (Some(rel_path), false, false, true) => format!(
            "`{rel_path}` invokes `syncpack lint`, but not in a supported fail-closed app script position, and does not list `syncpack` in dependencies or devDependencies. Add `syncpack` and remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden."
        ),
        (Some(rel_path), false, false, false) => format!(
            "`{rel_path}` does not list `syncpack` in dependencies or devDependencies and does not run `syncpack lint` in any app script. Add `syncpack` and wire `syncpack lint` so dependency policy is enforced by Syncpack instead of G3TS parsing npm version semantics."
        ),
        (Some(rel_path), false, true, _) => format!(
            "`{rel_path}` runs `syncpack lint` but does not list `syncpack` in dependencies or devDependencies. Add `syncpack` so the app uses the repo-pinned validator."
        ),
        (Some(rel_path), true, false, true) => format!(
            "`{rel_path}` invokes `syncpack lint`, but not in a supported fail-closed app script position. Remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden."
        ),
        (Some(rel_path), true, false, false) => format!(
            "`{rel_path}` lists `syncpack` but does not run `syncpack lint` in any app script. Add `syncpack lint` to the script surface so the package policy validator actually runs."
        ),
        (Some(rel_path), true, true, _) => format!(
            "`{rel_path}` has the required Syncpack package-policy setup, but the success path was not reached. Check the Astro Syncpack contract implementation."
        ),
        (None, _, _, _) => "The Syncpack package-policy contract could not be checked because `package.json` was not available. Restore the app package manifest, add `syncpack`, and wire `syncpack lint` there.".to_owned(),
    };

    results.push(crate::support::error(
        ID,
        "Syncpack package policy validator is not installed and wired",
        message,
        rel_path,
    ));
}
