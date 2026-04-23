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
                "`{rel_path}` does not run `astro check` in any app script. Add `astro check` to the script surface in `{rel_path}`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked."
            ),
            None => "The Astro check contract could not be verified because `package.json` was not available. Restore the app package manifest and add an `astro check` script there. Without that manifest, the app has no normal script surface where Astro diagnostics can be required.".to_owned(),
        };
        results.push(crate::support::error(
            ID,
            "Astro app scripts do not run `astro check`",
            message,
            rel_path,
        ));
    }
}
