use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-02";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        if crate::support::package_has_script_fragment(contract, "astro check") {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro check present",
                    format!("`{rel_path}` invokes `astro check` in the app script surface."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "Could not prove a real `astro check` invocation from `{rel_path}`. Add `astro check` to the app script surface."
            ),
            None => {
                "Could not prove a real `astro check` invocation because no package manifest was available."
                    .to_owned()
            }
        };
        results.push(crate::support::error(ID, "astro check missing", message, rel_path));
    }
}
