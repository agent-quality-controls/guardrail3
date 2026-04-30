use g3ts_astro_seo_types::G3TsAstroSeoEslintDirectiveInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-astro-seo/eslint-disable-inventory";

pub(crate) fn check_all(
    directives: &[G3TsAstroSeoEslintDirectiveInput],
    results: &mut Vec<G3CheckResult>,
) {
    if directives.is_empty() {
        results.push(crate::support::info(
            ID,
            "SEO source contains no ESLint disable directives",
            "No ESLint disable directives were found in Astro SEO source lanes.".to_owned(),
            "eslint.config.mjs",
        ));
        return;
    }

    for directive in directives {
        check(directive, results);
    }
}

fn check(directive: &G3TsAstroSeoEslintDirectiveInput, results: &mut Vec<G3CheckResult>) {
    if let Some(reason) = directive.parse_error() {
        results.push(crate::support::error(
            ID,
            "SEO ESLint disable inventory cannot be parsed",
            format!(
                "`{}` could not be parsed for ESLint disable directives: {reason}. G3TS fails closed because hidden disables would bypass delegated Astro SEO rules.",
                directive.rel_path()
            ),
            Some(directive.rel_path()),
        ));
        return;
    }

    results.push(crate::support::warning(
        ID,
        "SEO source contains an ESLint disable directive",
        format!(
            "`{}` line {} contains `{}` for {}. ESLint disables are allowed only as visible escape hatches; keep the directive described and avoid disabling protected SEO rules.",
            directive.rel_path(),
            directive.line(),
            directive.directive_kind(),
            disabled_rules(directive),
        ),
        Some(directive.rel_path()),
    ));
}

fn disabled_rules(directive: &G3TsAstroSeoEslintDirectiveInput) -> String {
    if directive.all_rules() {
        return "all rules".to_owned();
    }
    directive.disabled_rules().join(", ")
}
