use g3ts_astro_mdx_types::G3TsAstroMdxEslintDirectiveInput;
use guardrail3_check_types::G3CheckResult;

/// Internal constant `ID`.
const ID: &str = "g3ts-astro-mdx/eslint-disable-inventory";

/// Internal function `check_all`.
pub(crate) fn check_all(
    directives: &[G3TsAstroMdxEslintDirectiveInput],
    results: &mut Vec<G3CheckResult>,
) {
    if directives.is_empty() {
        results.push(crate::support::info(
            ID,
            "MDX source contains no ESLint disable directives",
            "No ESLint disable directives were found in Astro MDX lanes.".to_owned(),
            "eslint.config.mjs",
        ));
        return;
    }

    for directive in directives {
        check(directive, results);
    }
}

/// Internal function `check`.
fn check(directive: &G3TsAstroMdxEslintDirectiveInput, results: &mut Vec<G3CheckResult>) {
    if let Some(reason) = &directive.parse_error {
        results.push(crate::support::error(
            ID,
            "MDX ESLint disable inventory cannot be parsed",
            format!(
                "`{}` could not be parsed for ESLint disable directives: {reason}. G3TS fails closed because hidden disables would bypass delegated Astro MDX rules.",
                directive.rel_path
            ),
            Some(&directive.rel_path),
        ));
        return;
    }

    results.push(crate::support::warning(
        ID,
        "MDX source contains an ESLint disable directive",
        format!(
            "`{}` line {} contains `{}` for {}. ESLint disables are allowed only as visible escape hatches; keep the directive described and avoid disabling protected MDX rules.",
            directive.rel_path,
            directive.line,
            directive.directive_kind,
            disabled_rules(directive),
        ),
        Some(&directive.rel_path),
    ));
}

/// Internal function `disabled_rules`.
fn disabled_rules(directive: &G3TsAstroMdxEslintDirectiveInput) -> String {
    if directive.all_rules {
        return "all rules".to_owned();
    }
    directive.disabled_rules.join(", ")
}
