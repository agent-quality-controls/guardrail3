use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_seo_types::{G3TsAstroSeoPolicySnapshot, G3TsAstroSeoPolicySurfaceState};

/// `GUARDRAIL_CONFIG_REL_PATH` constant.
const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";

/// `ingest_seo_policy_surface`: ingest seo policy surface.
pub(crate) fn ingest_seo_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroSeoPolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = exact_included_file(crawl, &rel_path) else {
        return G3TsAstroSeoPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroSeoPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroSeoPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroSeoPolicySurfaceState::Parsed {
        snapshot: G3TsAstroSeoPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            metadata_helpers: astro.seo.metadata_helpers,
            json_ld_helpers: astro.seo.json_ld_helpers,
            strict_ai_readable: astro.seo.strict_ai_readable,
            llms_required_sections: astro.seo.llms_required_sections,
            llms_required_links: astro.seo.llms_required_links,
        },
    }
}

/// `exact_included_file`: exact included file.
fn exact_included_file<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == G3WorkspaceEntryKind::File
            && entry.ignore_state == G3WorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}
