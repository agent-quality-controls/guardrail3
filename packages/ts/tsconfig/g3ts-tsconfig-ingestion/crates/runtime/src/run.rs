use g3_workspace_crawl::{G3WorkspaceCrawl, root_file};
use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState};
use tsconfig_json_parser::{from_path_document, parse_error_reason};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsTsconfigChecksInput {
    let Some(entry) = root_file(crawl, "tsconfig.json") else {
        return G3TsTsconfigChecksInput {
            config: G3TsTsconfigState::Missing,
        };
    };

    if !entry.readable {
        return G3TsTsconfigChecksInput {
            config: G3TsTsconfigState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: "workspace crawl marked the selected config unreadable".to_owned(),
            },
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsTsconfigChecksInput {
                config: G3TsTsconfigState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: err.to_string(),
                },
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsTsconfigChecksInput {
            config: G3TsTsconfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: reason.to_owned(),
            },
        };
    }

    let (extends_chain, effective_compiler_options) =
        crate::resolve::build_extends_chain(&crawl.root_abs_path, &entry.path.abs_path, &document);

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: entry.path.rel_path.clone(),
            document,
            extends_chain,
            effective_compiler_options,
        },
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
