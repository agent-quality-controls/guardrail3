use g3ts_astro_i18n_types::{
    G3TsAstroI18nEslintPluginContractInput, G3TsAstroI18nEslintSurfaceState,
};
use guardrail3_check_types::G3CheckResult;

const I18NEXT_ID: &str = "g3ts-astro-i18n/i18next-plugin-wired";
const POLICY_ID: &str = "g3ts-astro-i18n/i18n-policy-plugin-wired";
const LINK_RULE_ID: &str = "g3ts-astro-i18n/no-unlocalized-internal-hrefs-rule";
const FORMAT_ID: &str = "g3ts-astro-i18n/raw-date-number-formatting-bans";
const DISABLE_ID: &str = "g3ts-astro-i18n/protected-i18n-rule-disables-restricted";
const I18NEXT_RULE: &str = "i18next/no-literal-string";
const LINK_RULE: &str = "astro-i18n-policy/no-unlocalized-internal-hrefs";
const REQUIRED_SELECTORS: [&str; 4] = [
    "CallExpression[callee.property.name='toLocaleDateString']",
    "CallExpression[callee.property.name='toLocaleString']",
    "NewExpression[callee.object.name='Intl'][callee.property.name='DateTimeFormat']",
    "NewExpression[callee.object.name='Intl'][callee.property.name='NumberFormat']",
];
const PROTECTED_DISABLES: [&str; 3] = [
    "i18next/no-literal-string",
    "astro-i18n-policy/*",
    "no-restricted-syntax",
];

pub(crate) fn check(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_i18next(contract, results);
    check_policy_plugin(contract, results);
    check_rule(contract, results, LINK_RULE_ID, LINK_RULE);
    check_formatting_bans(contract, results);
    check_disable_protection(contract, results);
}

fn check_i18next(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        snapshot
            .public_plugins
            .iter()
            .any(|plugin| plugin == "i18next")
            && snapshot
                .public_plugin_package_names
                .get("i18next")
                .is_some_and(|packages| {
                    packages
                        .iter()
                        .any(|package| package == "eslint-plugin-i18next")
                })
            && snapshot
                .public_error_rules
                .iter()
                .any(|rule| rule == I18NEXT_RULE)
    }) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                I18NEXT_ID,
                "i18next public-copy rule is wired",
                format!("`{rel_path}` activates `{I18NEXT_RULE}` at error severity on the Astro i18n public source probe."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        I18NEXT_ID,
        "i18next public-copy rule is not wired",
        format!(
            "`{}` must activate plugin `i18next` from `eslint-plugin-i18next` and `{I18NEXT_RULE}` at `error` on `[ts.astro.i18n].public_source_globs`.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn check_policy_plugin(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        snapshot
            .public_plugins
            .iter()
            .any(|plugin| plugin == "astro-i18n-policy")
            && snapshot
                .public_plugin_package_names
                .get("astro-i18n-policy")
                .is_some_and(|packages| {
                    packages
                        .iter()
                        .any(|package| package == "g3ts-eslint-plugin-astro-i18n-policy")
                })
    }) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                POLICY_ID,
                "Astro i18n policy plugin is wired",
                format!("`{rel_path}` activates `astro-i18n-policy` from `g3ts-eslint-plugin-astro-i18n-policy`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        POLICY_ID,
        "Astro i18n policy plugin is not wired",
        format!(
            "`{}` must activate plugin namespace `astro-i18n-policy` from `g3ts-eslint-plugin-astro-i18n-policy` on `[ts.astro.i18n].public_source_globs`.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn check_rule(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    rule_name: &str,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        snapshot
            .public_i18n_policy_rules
            .iter()
            .any(|rule| rule == rule_name)
    }) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                id,
                "Astro i18n policy rule is effective",
                format!(
                    "`{rel_path}` activates `{rule_name}` at error severity with explicit options."
                ),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        id,
        "Astro i18n policy rule is not effective",
        format!(
            "`{}` must activate `{rule_name}` at `error` with explicit options matching `[ts.astro.i18n]`: `locales`, `defaultLocale` only when `default_locale` is configured, `requireLocalePrefixForContentRoutes`, `allowedUnprefixedRoutes`, `contentRoutePrefixes`, `checkedInternalLinkHelpers`, `approvedInternalLinkHelpers`, and `approvedLocalizedLinkComponents`.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn check_formatting_bans(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        REQUIRED_SELECTORS.iter().all(|selector| {
            snapshot
                .public_no_restricted_syntax_selectors
                .iter()
                .any(|candidate| candidate == selector)
        }) && !snapshot
            .helper_no_restricted_syntax_selectors
            .iter()
            .any(|selector| {
                REQUIRED_SELECTORS
                    .iter()
                    .any(|required| required == selector)
            })
    }) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                FORMAT_ID,
                "Raw date and number formatting bans are wired",
                format!("`{rel_path}` bans raw locale formatting on public source probes and keeps approved helper probes unblocked."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        FORMAT_ID,
        "Raw date and number formatting bans are not wired",
        format!(
            "`{}` must configure `no-restricted-syntax` at `error` on public source probes for toLocaleDateString, toLocaleString, Intl.DateTimeFormat, and Intl.NumberFormat, while approved helper probes must not inherit those bans.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn check_disable_protection(
    contract: &G3TsAstroI18nEslintPluginContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::eslint_rel_path(&contract.config);
    if public_config(contract).is_some_and(|snapshot| {
        PROTECTED_DISABLES.iter().all(|pattern| {
            snapshot
                .public_restricted_disable_patterns
                .iter()
                .any(|candidate| candidate == pattern)
        })
    }) {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                DISABLE_ID,
                "Astro i18n delegated rule disables are restricted",
                format!("`{rel_path}` protects i18n delegated rules with `@eslint-community/eslint-comments/no-restricted-disable`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        DISABLE_ID,
        "Astro i18n delegated rule disables are not restricted",
        format!(
            "`{}` must configure `@eslint-community/eslint-comments/no-restricted-disable` for `i18next/no-literal-string`, `astro-i18n-policy/*`, and `no-restricted-syntax`.",
            rel_path.unwrap_or("eslint.config.*")
        ),
        rel_path,
    ));
}

fn public_config(
    contract: &G3TsAstroI18nEslintPluginContractInput,
) -> Option<&g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceSnapshot> {
    match &contract.config {
        G3TsAstroI18nEslintSurfaceState::Parsed { snapshot }
            if snapshot.public_probe_present && !snapshot.public_probe_ignored =>
        {
            Some(snapshot)
        }
        G3TsAstroI18nEslintSurfaceState::Missing { .. }
        | G3TsAstroI18nEslintSurfaceState::Unreadable { .. }
        | G3TsAstroI18nEslintSurfaceState::ParseError { .. }
        | G3TsAstroI18nEslintSurfaceState::Parsed { .. } => None,
    }
}
