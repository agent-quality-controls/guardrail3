use g3_workspace_crawl::{G3WorkspaceCrawl, root_file};
use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootState, root_snapshot};
use npmrc_parser::{from_path_document, parse_error_reason, typed};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: if root_policy_applies(crawl) {
            ingest_root(crawl)
        } else {
            G3TsNpmrcRootState::NotPackageManagerRoot
        },
    }
}

/// Reads and parses the root `.npmrc` (if any), returning the corresponding state.
fn ingest_root(crawl: &G3WorkspaceCrawl) -> G3TsNpmrcRootState {
    let Some(entry) = root_file(crawl, ".npmrc") else {
        return G3TsNpmrcRootState::Missing;
    };

    if !entry.readable {
        return G3TsNpmrcRootState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected root .npmrc unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsNpmrcRootState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsNpmrcRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(snapshot) = typed(&document) else {
        return G3TsNpmrcRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed .npmrc document is not typed".to_owned(),
        };
    };
    G3TsNpmrcRootState::Parsed {
        snapshot: root_snapshot(&entry.path.rel_path, snapshot),
    }
}

/// Whether root `.npmrc` policy applies to this workspace (pnpm-rooted only).
fn root_policy_applies(crawl: &G3WorkspaceCrawl) -> bool {
    root_file(crawl, "pnpm-workspace.yaml").is_some()
        || root_file(crawl, "pnpm-lock.yaml").is_some()
}
