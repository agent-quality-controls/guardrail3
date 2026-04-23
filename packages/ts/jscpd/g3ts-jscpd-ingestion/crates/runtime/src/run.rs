use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, root_file};
use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState, root_snapshot};
use jscpd_json_parser::{from_path_document, parse_error_reason, typed};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsJscpdChecksInput {
    let Some(entry) = root_file(crawl, ".jscpd.json") else {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::Missing,
        };
    };

    if !entry.readable {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: "workspace crawl marked the selected root .jscpd.json unreadable"
                    .to_owned(),
            },
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsJscpdChecksInput {
                root: G3TsJscpdRootState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: err.to_string(),
                },
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: reason.to_owned(),
            },
        };
    }

    let snapshot = typed(&document).expect("parsed .jscpd.json document should stay typed");
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Parsed {
            snapshot: root_snapshot(&entry.path.rel_path, snapshot),
        },
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
