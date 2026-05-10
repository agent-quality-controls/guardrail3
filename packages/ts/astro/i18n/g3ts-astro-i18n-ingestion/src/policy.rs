use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_i18n_types::{G3TsAstroI18nPolicySnapshot, G3TsAstroI18nPolicySurfaceState};

/// Relative path of the guardrail TS config within an Astro app root.
const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";

/// Ingests the i18n policy surface from an Astro app's `guardrail3-ts.toml`.
pub(crate) fn ingest_i18n_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroI18nPolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsAstroI18nPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroI18nPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroI18nPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroI18nPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroI18nPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    let Some(i18n) = astro.i18n else {
        return G3TsAstroI18nPolicySurfaceState::MissingI18nPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroI18nPolicySurfaceState::Parsed {
        snapshot: G3TsAstroI18nPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            locales: i18n.locales,
            default_locale: i18n.default_locale,
            require_locale_prefix_for_content_routes: i18n.require_locale_prefix_for_content_routes,
            allowed_unprefixed_routes: i18n.allowed_unprefixed_routes,
            content_route_prefixes: i18n.content_route_prefixes,
            checked_internal_link_helpers: i18n.checked_internal_link_helpers,
            approved_internal_link_helpers: i18n.approved_internal_link_helpers,
            approved_localized_link_components: i18n.approved_localized_link_components,
            approved_date_format_helpers: i18n.approved_date_format_helpers,
            approved_number_format_helpers: i18n.approved_number_format_helpers,
            public_source_globs: i18n.public_source_globs,
            helper_source_globs: i18n.helper_source_globs,
        },
    }
}
