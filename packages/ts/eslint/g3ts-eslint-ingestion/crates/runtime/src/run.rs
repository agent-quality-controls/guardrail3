use eslint_config_parser::{parse_document, parse_error_reason};
use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsEslintConfigChecksInput {
    let Some(entry) = crate::select::select_active_root_eslint_config(crawl) else {
        return G3TsEslintConfigChecksInput {
            config: G3TsEslintConfigState::Missing,
        };
    };

    if !entry.readable {
        return G3TsEslintConfigChecksInput {
            config: G3TsEslintConfigState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: "workspace crawl marked the selected config unreadable".to_owned(),
            },
        };
    }

    let probes = crate::select::probe_targets(crawl, &entry.path.rel_path);
    let document = match parse_document(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
        Ok(document) => document,
        Err(err) => {
            return G3TsEslintConfigChecksInput {
                config: G3TsEslintConfigState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: err.to_string(),
                },
            };
        }
    };

    let config = match parse_error_reason(&document) {
        Some(reason) => G3TsEslintConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        },
        None => G3TsEslintConfigState::Parsed {
            rel_path: entry.path.rel_path.clone(),
            document,
        },
    };

    G3TsEslintConfigChecksInput { config }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod run_tests;
