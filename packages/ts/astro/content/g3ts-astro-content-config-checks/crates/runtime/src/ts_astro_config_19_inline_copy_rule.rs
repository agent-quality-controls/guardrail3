use g3ts_astro_types::G3TsAstroContentConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONTENT-CONFIG-19";
const DEPENDENCY_NAME: &str = "eslint-plugin-i18next";
const PLUGIN_NAME: &str = "i18next";
const RULE_NAME: &str = "i18next/no-literal-string";

pub(crate) fn check(input: &G3TsAstroContentConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = g3ts_astro_check_support::core::package_rel_path(contract);
        if !g3ts_astro_check_support::core::package_has_dependency(contract, DEPENDENCY_NAME) {
            results.push(g3ts_astro_check_support::core::error(
                ID,
                "Inline public-copy ESLint plugin package is missing",
                format!(
                    "`{}` must list `{DEPENDENCY_NAME}` in dependencies or devDependencies. The Astro family delegates hardcoded public-copy detection to `{RULE_NAME}`.",
                    rel_path.unwrap_or("package.json")
                ),
                rel_path,
            ));
        }
    }

    for contract in &input.eslint_contracts {
        let rel_path = g3ts_astro_check_support::eslint::eslint_rel_path(contract);
        if g3ts_astro_check_support::eslint::eslint_required_lanes_have_inline_public_content_rule(
            contract,
            PLUGIN_NAME,
            RULE_NAME,
        ) {
            if let Some(rel_path) = rel_path {
                results.push(g3ts_astro_check_support::core::info(
                    ID,
                    "Inline public-copy ESLint rule is effective",
                    format!("`{rel_path}` enforces `{RULE_NAME}` with the exact strict Astro public-copy options on Astro, TS, and TSX source probes."),
                    rel_path,
                ));
            }
            continue;
        }

        results.push(g3ts_astro_check_support::core::error(
            ID,
            "Inline public-copy ESLint rule is not effective",
            format!(
                "`{}` must activate plugin `{PLUGIN_NAME}` and rule `{RULE_NAME}` at `error` on Astro, TS, and TSX source probes with the exact strict options from the Astro delegation plan. Missing probes, ignored probes, broad allowlists, or changed messages fail this contract.",
                rel_path.unwrap_or("eslint.config.*")
            ),
            rel_path,
        ));
    }
}
