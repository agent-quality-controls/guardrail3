use std::fs::File;
use std::path::{Path, PathBuf};

use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, root_file};
use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState, root_snapshot};
use jscpd_json_parser::{from_path_document, parse_error_reason, typed};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsJscpdChecksInput {
    let Some(entry) = select_root_jscpd(crawl) else {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::Missing,
        };
    };

    if !entry.readable {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::Unreadable {
                rel_path: entry.rel_path.clone(),
                reason: entry.reason.clone().unwrap_or_else(|| {
                    "workspace crawl marked the selected root .jscpd.json unreadable".to_owned()
                }),
            },
        };
    }

    let document = match from_path_document(&entry.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsJscpdChecksInput {
                root: G3TsJscpdRootState::ParseError {
                    rel_path: entry.rel_path.clone(),
                    reason: err.to_string(),
                },
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::ParseError {
                rel_path: entry.rel_path.clone(),
                reason: reason.to_owned(),
            },
        };
    }

    let snapshot = typed(&document).expect("parsed .jscpd.json document should stay typed");
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Parsed {
            snapshot: root_snapshot(&entry.rel_path, snapshot),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedRootJscpd {
    abs_path: PathBuf,
    rel_path: String,
    readable: bool,
    reason: Option<String>,
}

fn select_root_jscpd(crawl: &G3WorkspaceCrawl) -> Option<SelectedRootJscpd> {
    if let Some(entry) = root_file(crawl, ".jscpd.json") {
        return Some(SelectedRootJscpd {
            abs_path: entry.path.abs_path.clone(),
            rel_path: entry.path.rel_path.clone(),
            readable: entry.readable,
            reason: None,
        });
    }

    find_ancestor_root_jscpd(crawl)
}

fn find_ancestor_root_jscpd(crawl: &G3WorkspaceCrawl) -> Option<SelectedRootJscpd> {
    let mut current_dir = crawl.root_abs_path.parent();

    while let Some(dir) = current_dir {
        let candidate = dir.join(".jscpd.json");
        if candidate.is_file() {
            let rel_path = relative_ancestor_config_path(&crawl.root_abs_path, dir);
            let readable = File::open(&candidate).is_ok();
            let reason = (!readable).then(|| {
                "selected ancestor root .jscpd.json could not be opened from the validation root"
                    .to_owned()
            });
            return Some(SelectedRootJscpd {
                abs_path: candidate,
                rel_path,
                readable,
                reason,
            });
        }
        current_dir = dir.parent();
    }

    None
}

fn relative_ancestor_config_path(validation_root: &Path, config_dir: &Path) -> String {
    let mut rel_path = PathBuf::new();
    let mut current = validation_root;

    while current != config_dir {
        rel_path.push("..");
        current = current.parent().expect(
            "ancestor config selection should only compare directories on the validation root chain",
        );
    }

    rel_path.push(".jscpd.json");
    rel_path.to_string_lossy().into_owned()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
