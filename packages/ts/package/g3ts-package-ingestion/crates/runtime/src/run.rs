use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind, root_file,
};
use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalState, G3TsPackageRootState, local_snapshot,
    root_snapshot,
};
use package_json_parser::{from_path_document, parse_error_reason, typed};

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsPackageChecksInput {
    let root_policy_applies = root_policy_applies(crawl);

    G3TsPackageChecksInput {
        root: if root_policy_applies {
            ingest_root(crawl)
        } else {
            G3TsPackageRootState::NotPackageManagerRoot
        },
        locals: crawl
            .entries
            .iter()
            .filter(|entry| is_local_package_json(entry, root_policy_applies))
            .map(ingest_local)
            .collect(),
    }
}

fn ingest_root(crawl: &G3WorkspaceCrawl) -> G3TsPackageRootState {
    let Some(entry) = root_file(crawl, "package.json") else {
        return G3TsPackageRootState::Missing;
    };

    if !entry.readable {
        return G3TsPackageRootState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected root manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsPackageRootState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsPackageRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let snapshot = typed(&document).expect("parsed package.json document should stay typed");
    G3TsPackageRootState::Parsed {
        snapshot: root_snapshot(&entry.path.rel_path, snapshot),
    }
}

fn ingest_local(entry: &G3WorkspaceEntry) -> G3TsPackageLocalState {
    if !entry.readable {
        return G3TsPackageLocalState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the local manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsPackageLocalState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsPackageLocalState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let snapshot = typed(&document).expect("parsed package.json document should stay typed");
    G3TsPackageLocalState::Parsed {
        snapshot: local_snapshot(&entry.path.rel_path, snapshot),
    }
}

fn is_local_package_json(entry: &G3WorkspaceEntry, root_policy_applies: bool) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && if root_policy_applies {
            entry.path.rel_path.ends_with("/package.json")
        } else {
            entry.path.rel_path == "package.json" || entry.path.rel_path.ends_with("/package.json")
        }
}

fn root_policy_applies(crawl: &G3WorkspaceCrawl) -> bool {
    root_file(crawl, "pnpm-workspace.yaml").is_some()
        || root_file(crawl, "pnpm-lock.yaml").is_some()
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
