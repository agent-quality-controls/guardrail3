use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SETUP-CONFIG-05";
const PLUGIN_NAME: &str = "astro";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        let rel_path = g3ts_astro_check_support::eslint::eslint_rel_path(contract);
        if g3ts_astro_check_support::eslint::eslint_astro_source_has_plugin(contract, PLUGIN_NAME) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "astro ESLint plugin wired",
                    format!("`{rel_path}` activates `{PLUGIN_NAME}` for the required Astro source probe."),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` from `eslint-plugin-astro` on the required Astro source probe. Add the `astro` plugin from `eslint-plugin-astro` to the Astro file lane in `{rel_path}`. Astro files must run through the Astro plugin so framework lint rules actually execute."
            ),
            None => "The Astro ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and wire the `astro` plugin from `eslint-plugin-astro` for Astro files there. Astro files must run through the Astro plugin so framework lint rules actually execute.".to_owned(),
        };
        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Astro ESLint source probe is not wired to `eslint-plugin-astro`",
            message,
            rel_path,
        ));
    }
}
