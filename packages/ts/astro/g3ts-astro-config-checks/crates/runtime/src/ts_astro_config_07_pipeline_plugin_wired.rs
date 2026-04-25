use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-07";
const PLUGIN_NAME: &str = "astro-pipeline";
const INLINE_PUBLIC_CONTENT_PLUGIN_NAME: &str = "i18next";
const INLINE_PUBLIC_CONTENT_RULE: &str = "i18next/no-literal-string";
const REQUIRED_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-runtime-mdx-eval",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const ROUTE_SCOPED_REQUIRED_RULES: [&str; 7] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_REQUIRED_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];
const CONTENT_SOURCE_REQUIRED_RULES: [&str; 3] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
];

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.eslint_contracts {
        if !contract.requires_source_pipeline_linting {
            continue;
        }

        let rel_path = crate::support::eslint_rel_path(contract);
        let has_pipeline_rules = crate::support::eslint_required_lanes_have_effective_pipeline_rules(
            contract,
            PLUGIN_NAME,
            &REQUIRED_RULES,
            &ROUTE_SCOPED_REQUIRED_RULES,
            &CONTENT_DATA_REQUIRED_RULES,
            &CONTENT_SOURCE_REQUIRED_RULES,
        );
        let has_inline_public_content_rule =
            crate::support::eslint_required_lanes_have_inline_public_content_rule(
                contract,
                INLINE_PUBLIC_CONTENT_PLUGIN_NAME,
                INLINE_PUBLIC_CONTENT_RULE,
            );

        if has_pipeline_rules && has_inline_public_content_rule {
            if let Some(rel_path) = rel_path {
                results.push(crate::support::info(
                    ID,
                    "Astro content ESLint plugins are wired and effective",
                    format!(
                        "`{rel_path}` activates `{PLUGIN_NAME}` and `{INLINE_PUBLIC_CONTENT_PLUGIN_NAME}` and enforces the required Astro pipeline rules plus `{INLINE_PUBLIC_CONTENT_RULE}` at error severity on the Astro, TS, and TSX source lanes. The route-scoped rules have route coverage for Astro page routes and endpoint coverage for Astro endpoints; the content-data rule has non-empty `contentDataModuleGlobs`; the authored-content rules have non-empty `authoredContentGlobs` or `specContentGlobs`; and `{INLINE_PUBLIC_CONTENT_RULE}` uses `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message without broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that would hide authored copy."
                    ),
                    rel_path,
                ));
            }
            continue;
        }

        let message = match rel_path {
            Some(rel_path) => format!(
                "`{rel_path}` does not activate `{PLUGIN_NAME}` and `{INLINE_PUBLIC_CONTENT_PLUGIN_NAME}` with all required Astro content-pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable `{PLUGIN_NAME}` with the required Astro pipeline rules, route coverage for Astro page routes, endpoint coverage for Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, and non-empty `authoredContentGlobs` or `specContentGlobs` on `astro-pipeline/no-authored-content-fs-read`, `astro-pipeline/no-authored-content-glob`, and `astro-pipeline/no-authored-content-imports`. Also enable `{INLINE_PUBLIC_CONTENT_PLUGIN_NAME}` and `{INLINE_PUBLIC_CONTENT_RULE}` with `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message; do not add broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that hide authored copy. Without this delegated literal-string rule, agents can hardcode public landing copy in routes, UI components, or source data objects while the Astro pipeline checks still pass."
            ),
            None => format!(
                "The Astro pipeline ESLint wiring contract could not be checked because `eslint.config.*` was not available. Restore the app ESLint config and enable `{PLUGIN_NAME}` with the required rules, route coverage for Astro page routes, endpoint coverage for Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, and non-empty `authoredContentGlobs` or `specContentGlobs` on the authored-content rules on the Astro, TS, and TSX source lanes there. Also enable `{INLINE_PUBLIC_CONTENT_PLUGIN_NAME}` and `{INLINE_PUBLIC_CONTENT_RULE}` with `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message without broad allowlists that hide authored copy. Without the delegated literal-string rule, agents can hardcode public landing copy in source while the Astro pipeline checks still pass."
            ),
        };
        results.push(crate::support::error(
            ID,
            "Astro ESLint lanes are not enforcing the required content rules",
            message,
            rel_path,
        ));
    }
}
