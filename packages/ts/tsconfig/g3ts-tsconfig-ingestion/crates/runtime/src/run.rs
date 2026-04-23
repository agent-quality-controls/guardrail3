use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry, root_file,
};
use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState, inline_strict_flags};
use tsconfig_json_parser::{from_path_document, parse_error_reason};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsTsconfigChecksInput {
    let Some(entry) = select_root_tsconfig(crawl) else {
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
            uses_extends: !tsconfig_json_parser::extends_entries(&document).is_empty(),
            extends_chain,
            inline_strict_flags: inline_strict_flags(&document),
            effective_compiler_options,
        },
    }
}

fn select_root_tsconfig<'a>(crawl: &'a G3WorkspaceCrawl) -> Option<&'a G3WorkspaceEntry> {
    root_file(crawl, "tsconfig.json").or_else(|| root_file(crawl, "tsconfig.base.json"))
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
