use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-01";
const PACKAGE_NAME: &str = "astro";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);
        if crate::support::package_has_dependency(contract, PACKAGE_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro package present",
                    format!("`{rel_path}` includes `{PACKAGE_NAME}`."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!("`{rel_path}` does not include `{PACKAGE_NAME}`."),
            None => {
                "Could not verify the Astro package contract because no package manifest was available."
                    .to_owned()
            }
        };

        results.push(crate::support::error(
            ID,
            "astro package missing",
            message,
            rel_path,
        ));
    }
}
