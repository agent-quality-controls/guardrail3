use cargo_toml_parser::{LintValue, parse};
use g3rs_code_ingestion_types::G3RsCodeUnsafeCodeLintFact;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::run::IngestionError;

pub(crate) fn collect_unsafe_code_lints(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeUnsafeCodeLintFact>, IngestionError> {
    let mut lints = Vec::new();

    for entry in crate::config_scope::select_owned_cargo_entries(crawl)? {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let content =
            crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

        let cargo = parse(&content).map_err(|err| IngestionError::ParseFailed {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;

        let Some(workspace) = cargo.workspace else {
            continue;
        };

        let lint_level = workspace
            .lints
            .and_then(|lints| lints.tools.get("rust").cloned())
            .and_then(|tool| tool.get("unsafe_code").cloned())
            .and_then(|value| match value {
                LintValue::Level(level) => Some(level),
                LintValue::Detailed(detail) => Some(detail.level),
            });

        lints.push(G3RsCodeUnsafeCodeLintFact {
            cargo_rel_path: entry.path.rel_path.clone(),
            lint_level,
        });
    }

    Ok(lints)
}
