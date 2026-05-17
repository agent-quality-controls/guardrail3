use std::path::{Path, PathBuf};

use g3_workspace_crawl::{G3WorkspaceCrawl, root_file};
use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState, root_snapshot};
use jscpd_json_parser::{from_path_document, parse_error_reason, typed};

/// Ingest the workspace crawl into a `G3TsJscpdChecksInput` describing the
/// selected root `.jscpd.json` configuration.
///
/// The function picks the workspace root config when present and otherwise
/// walks ancestor directories of the validation root looking for a
/// `.jscpd.json` file, recording readability and parse status for downstream
/// checks.
#[must_use]
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
                reason: entry.reason.unwrap_or_else(|| {
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
                    rel_path: entry.rel_path,
                    reason: err.to_string(),
                },
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::ParseError {
                rel_path: entry.rel_path,
                reason: reason.to_owned(),
            },
        };
    }

    let Some(snapshot) = typed(&document) else {
        return G3TsJscpdChecksInput {
            root: G3TsJscpdRootState::ParseError {
                rel_path: entry.rel_path,
                reason: "parsed .jscpd.json document did not yield a typed snapshot".to_owned(),
            },
        };
    };
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Parsed {
            snapshot: root_snapshot(&entry.rel_path, snapshot),
        },
    }
}

/// Selected root `.jscpd.json` candidate gathered by the workspace scan.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedRootJscpd {
    /// Absolute path to the candidate config file.
    abs_path: PathBuf,
    /// Path of the candidate config file relative to the validation root.
    rel_path: String,
    /// Whether the candidate was openable for reading.
    readable: bool,
    /// Optional human-readable reason describing why the candidate was not
    /// readable, when applicable.
    reason: Option<String>,
}

/// Returns the workspace-root `.jscpd.json`, or the closest ancestor copy if
/// the workspace itself does not declare one.
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

/// Walks ancestor directories of the validation root, returning the first
/// `.jscpd.json` file encountered together with its readability state.
fn find_ancestor_root_jscpd(crawl: &G3WorkspaceCrawl) -> Option<SelectedRootJscpd> {
    let mut current_dir = crawl.root_abs_path.parent();

    while let Some(dir) = current_dir {
        let candidate = dir.join(".jscpd.json");
        if candidate.is_file() {
            let rel_path = relative_ancestor_config_path(&crawl.root_abs_path, dir);
            let readable = crate::fs::is_readable_file(&candidate);
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

/// Builds the `..`-prefixed path string from `validation_root` up to the
/// directory holding the selected ancestor `.jscpd.json`.
///
/// The walk stops as soon as `current` equals `config_dir`; if the parent
/// chain is exhausted before that happens the path falls back to a single
/// `.jscpd.json` segment so the caller still receives a usable string.
fn relative_ancestor_config_path(validation_root: &Path, config_dir: &Path) -> String {
    let mut rel_path = PathBuf::new();
    let mut current = validation_root;

    while current != config_dir {
        rel_path.push("..");
        let Some(parent) = current.parent() else {
            break;
        };
        current = parent;
    }

    rel_path.push(".jscpd.json");
    rel_path.to_string_lossy().into_owned()
}
