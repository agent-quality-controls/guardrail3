use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-05";
const PLUGIN_NAME: &str = "astro";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        let rel_path = crate::support::eslint_rel_path(contract);
        if crate::support::eslint_required_lanes_have_plugin(contract, PLUGIN_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "astro ESLint plugin wired",
                    format!("`{rel_path}` activates `{PLUGIN_NAME}` for the required Astro source lanes."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` on the required Astro source lanes. Add the `astro` plugin to the Astro, TS, and TSX lane configs in `{rel_path}`. Astro source files must run through the Astro plugin so framework lint rules actually execute."
            ),
            None => "The Astro ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and wire the `astro` plugin there. Astro source files must run through the Astro plugin so framework lint rules actually execute.".to_owned(),
        };
        results.push(crate::support::error(
            ID,
            "Astro ESLint lanes are not wired to the `astro` plugin",
            message,
            rel_path,
        ));
    }
}
