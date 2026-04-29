use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_media_types::{G3TsAstroMediaPolicySnapshot, G3TsAstroMediaPolicySurfaceState};

const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";

pub(crate) fn ingest_media_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroMediaPolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsAstroMediaPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroMediaPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroMediaPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroMediaPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroMediaPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    let Some(media) = astro.media else {
        return G3TsAstroMediaPolicySurfaceState::MissingMediaPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroMediaPolicySurfaceState::Parsed {
        snapshot: G3TsAstroMediaPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            favicon: media.favicon,
            app_icons: media.app_icons,
            default_social_image: media.default_social_image,
            allow_svg_icons: media.allow_svg_icons,
            public_source_globs: media.public_source_globs,
            media_helper_modules: media.media_helper_modules,
            approved_media_helpers: media.approved_media_helpers,
            content_image_components: media.content_image_components,
            content_image_key_props: media.content_image_key_props,
            banned_image_source_props: media.banned_image_source_props,
            banned_image_alt_props: media.banned_image_alt_props,
            allowed_public_image_paths: media.allowed_public_image_paths,
            checked_image_extensions: media.checked_image_extensions,
            metadata_image_property_names: media.metadata_image_property_names,
        },
    }
}
