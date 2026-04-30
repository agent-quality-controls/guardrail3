use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_style_types::{G3TsStylePolicySnapshot, G3TsStylePolicySurfaceState};

const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";

pub(crate) fn ingest_policy(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsStylePolicySurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, GUARDRAIL_CONFIG_REL_PATH);
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsStylePolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsStylePolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match guardrail3_rs_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsStylePolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(ts) = config.ts else {
        return G3TsStylePolicySurfaceState::MissingTsPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(style) = ts.style else {
        return G3TsStylePolicySurfaceState::MissingStylePolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    G3TsStylePolicySurfaceState::Parsed {
        snapshot: G3TsStylePolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            source_globs: style.source_globs,
            stylelint_css_globs: style.stylelint_css_globs,
            extra_fields: style.extra.keys().cloned().collect(),
        },
    }
}
