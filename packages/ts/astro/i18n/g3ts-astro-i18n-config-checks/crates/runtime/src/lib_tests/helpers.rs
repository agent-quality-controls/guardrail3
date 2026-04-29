use std::collections::BTreeMap;

use g3ts_astro_i18n_types::{
    G3TsAstroI18nConfigChecksInput, G3TsAstroI18nEslintPluginContractInput,
    G3TsAstroI18nEslintSurfaceSnapshot, G3TsAstroI18nEslintSurfaceState,
    G3TsAstroI18nIntegrationContractInput, G3TsAstroI18nPolicySnapshot,
    G3TsAstroI18nPolicySurfaceState, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState,
};

pub(super) fn golden() -> G3TsAstroI18nConfigChecksInput {
    G3TsAstroI18nConfigChecksInput {
        integration_contracts: vec![G3TsAstroI18nIntegrationContractInput {
            app_root_rel_path: "apps/landing".to_owned(),
            package: package(),
            astro_policy: policy(),
        }],
        eslint_contracts: vec![G3TsAstroI18nEslintPluginContractInput {
            app_root_rel_path: "apps/landing".to_owned(),
            config: eslint_config(),
            astro_policy: policy(),
        }],
    }
}

fn package() -> G3TsAstroPackageSurfaceState {
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: "apps/landing/package.json".to_owned(),
            dependencies: Vec::new(),
            dev_dependencies: vec![
                "eslint-plugin-i18next".to_owned(),
                "g3ts-eslint-plugin-astro-i18n-policy".to_owned(),
                "@eslint-community/eslint-plugin-eslint-comments".to_owned(),
            ],
        },
    }
}

fn policy() -> G3TsAstroI18nPolicySurfaceState {
    G3TsAstroI18nPolicySurfaceState::Parsed {
        snapshot: G3TsAstroI18nPolicySnapshot {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
            locales: vec!["en".to_owned()],
            default_locale: Some("en".to_owned()),
            require_locale_prefix_for_content_routes: true,
            allowed_unprefixed_routes: vec!["/".to_owned()],
            content_route_prefixes: vec!["/blog".to_owned()],
            checked_internal_link_helpers: vec!["buildPath".to_owned()],
            approved_internal_link_helpers: vec!["localizedHref".to_owned()],
            approved_localized_link_components: vec!["LocalizedLink".to_owned()],
            approved_date_format_helpers: vec!["src/i18n/format-date.ts".to_owned()],
            approved_number_format_helpers: vec!["src/i18n/format-number.ts".to_owned()],
            content_image_components: vec!["ArticleImage".to_owned()],
            content_image_key_props: vec!["image".to_owned()],
            banned_image_source_props: vec!["src".to_owned()],
            banned_image_alt_props: vec!["alt".to_owned()],
            public_source_globs: vec!["src/**/*.{astro,ts,tsx}".to_owned()],
            helper_source_globs: vec!["src/i18n/**/*.ts".to_owned()],
        },
    }
}

fn eslint_config() -> G3TsAstroI18nEslintSurfaceState {
    let mut packages = BTreeMap::new();
    let _ = packages.insert(
        "i18next".to_owned(),
        vec!["eslint-plugin-i18next".to_owned()],
    );
    let _ = packages.insert(
        "astro-i18n-policy".to_owned(),
        vec!["g3ts-eslint-plugin-astro-i18n-policy".to_owned()],
    );

    G3TsAstroI18nEslintSurfaceState::Parsed {
        snapshot: G3TsAstroI18nEslintSurfaceSnapshot {
            rel_path: "apps/landing/eslint.config.mjs".to_owned(),
            public_probe_present: true,
            public_probe_ignored: false,
            helper_probe_present: true,
            helper_probe_ignored: false,
            public_plugins: vec!["i18next".to_owned(), "astro-i18n-policy".to_owned()],
            public_plugin_package_names: packages,
            public_error_rules: vec![
                "i18next/no-literal-string".to_owned(),
                "no-restricted-syntax".to_owned(),
            ],
            public_restricted_disable_patterns: vec![
                "i18next/no-literal-string".to_owned(),
                "astro-i18n-policy/*".to_owned(),
                "no-restricted-syntax".to_owned(),
            ],
            public_i18n_policy_rules: vec![
                "astro-i18n-policy/no-unlocalized-internal-hrefs".to_owned(),
                "astro-i18n-policy/no-inline-image-alt".to_owned(),
                "astro-i18n-policy/require-content-image-key".to_owned(),
            ],
            public_no_restricted_syntax_selectors: vec![
                "CallExpression[callee.property.name='toLocaleDateString']".to_owned(),
                "CallExpression[callee.property.name='toLocaleString']".to_owned(),
                "NewExpression[callee.object.name='Intl'][callee.property.name='DateTimeFormat']"
                    .to_owned(),
                "NewExpression[callee.object.name='Intl'][callee.property.name='NumberFormat']"
                    .to_owned(),
            ],
            helper_no_restricted_syntax_selectors: Vec::new(),
        },
    }
}
