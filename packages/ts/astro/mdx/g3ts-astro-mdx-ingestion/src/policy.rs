use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState,
};
use g3ts_astro_mdx_types::{G3TsAstroMdxPolicySnapshot, G3TsAstroMdxPolicySurfaceState};

/// `GUARDRAIL_CONFIG_REL_PATH` constant.
const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";

/// `ingest_mdx_policy_surface` helper.
pub(crate) fn ingest_mdx_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroMdxPolicySurfaceState {
    let rel_path = g3ts_astro_check_support::surfaces::scoped_rel_path(
        app_root_rel_path,
        GUARDRAIL_CONFIG_REL_PATH,
    );
    let Some(entry) = exact_included_file(crawl, &rel_path) else {
        return G3TsAstroMdxPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroMdxPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsAstroMdxPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(astro) = ts.astro else {
        return G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsAstroMdxPolicySurfaceState::Parsed {
        snapshot: G3TsAstroMdxPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            content_root: astro.content.root,
            mdx_component_maps: astro.mdx.component_maps,
        },
    }
}

/// `exact_included_file` helper.
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
