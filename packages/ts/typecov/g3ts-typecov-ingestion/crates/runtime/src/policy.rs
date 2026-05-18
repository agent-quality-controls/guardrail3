use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_typecov_types::{G3TsTypecovPolicySnapshot, G3TsTypecovPolicySurfaceState};

/// Local policy filename for G3TS TypeScript workspace configuration.
const GUARDRAIL_CONFIG: &str = "guardrail3-ts.toml";

/// Ingests the typecov policy surface from `guardrail3-ts.toml`.
pub(crate) fn ingest_typecov_policy_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsTypecovPolicySurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, GUARDRAIL_CONFIG);
    let Some(entry) = crate::roots::exact_included_file(crawl, &rel_path) else {
        return G3TsTypecovPolicySurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsTypecovPolicySurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the guardrail config unreadable".to_owned(),
        };
    }

    let config = match g3ts_toml_parser::from_path(&entry.path.abs_path) {
        Ok(config) => config,
        Err(error) => {
            return G3TsTypecovPolicySurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    let Some(typecov) = config.typecov else {
        return G3TsTypecovPolicySurfaceState::MissingTypecovPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };

    let Some(minimum) = typecov.minimum else {
        return G3TsTypecovPolicySurfaceState::MissingTypecovPolicy {
            rel_path: entry.path.rel_path.clone(),
        };
    };
    let Some(minimum) = minimum
        .as_integer()
        .and_then(|value| u8::try_from(value).ok())
        .filter(|value| *value <= 100)
    else {
        return G3TsTypecovPolicySurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "typecov.minimum must be an integer in 0..=100".to_owned(),
        };
    };

    G3TsTypecovPolicySurfaceState::Parsed {
        snapshot: G3TsTypecovPolicySnapshot {
            rel_path: entry.path.rel_path.clone(),
            minimum,
        },
    }
}
