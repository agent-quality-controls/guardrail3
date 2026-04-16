use std::collections::BTreeSet;

use cargo_toml_parser::types::CargoToml;
use g3rs_apparch_types::{G3RsApparchCrate, G3RsApparchLayer, G3RsApparchRustPolicyState};
use glob::Pattern;
use guardrail3_rs_toml_parser::from_path as parse_guardrail3_rs_toml;

use super::error::G3RsApparchIngestionError;
use super::model::{CrateRecord, WorkspaceRoot};
use crate::view::CrawlView;

pub(super) fn load_workspace_root(
    view: &CrawlView<'_>,
) -> Result<WorkspaceRoot, G3RsApparchIngestionError> {
    let Some(entry) = view.entry("Cargo.toml") else {
        return Err(G3RsApparchIngestionError::CargoTomlNotFound);
    };
    if !entry.readable {
        return Err(G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = cargo_toml_parser::from_path(&entry.path.abs_path).map_err(|error| {
        G3RsApparchIngestionError::ParseFailed {
            path: entry.path.abs_path.clone(),
            reason: error.to_string(),
        }
    })?;

    if cargo.workspace.is_none() {
        return Err(G3RsApparchIngestionError::NormalizationFailed {
            path: entry.path.abs_path.clone(),
            reason: "root Cargo.toml must declare a [workspace] table".to_owned(),
        });
    }

    Ok(WorkspaceRoot {
        cargo,
        rust_policy: load_rust_policy(view),
    })
}

fn load_rust_policy(view: &CrawlView<'_>) -> G3RsApparchRustPolicyState {
    let Some(entry) = view.entry("guardrail3-rs.toml") else {
        return G3RsApparchRustPolicyState::Missing;
    };
    let rel_path = "guardrail3-rs.toml".to_owned();
    if !entry.readable {
        return G3RsApparchRustPolicyState::Unreadable {
            rel_path,
            reason: "file is not readable".to_owned(),
        };
    }

    match parse_guardrail3_rs_toml(&entry.path.abs_path) {
        Ok(parsed) => G3RsApparchRustPolicyState::Parsed {
            rel_path,
            profile: parsed.profile,
            allowed_deps: parsed.allowed_deps,
            waivers: parsed.waivers,
        },
        Err(error) => G3RsApparchRustPolicyState::ParseError {
            rel_path,
            reason: error.to_string(),
        },
    }
}

pub(super) fn collect_workspace_crates(
    view: &CrawlView<'_>,
    workspace: &WorkspaceRoot,
) -> Result<Vec<CrateRecord>, G3RsApparchIngestionError> {
    let mut rel_dirs = resolve_member_dirs(view, &workspace.cargo)?;
    if workspace.cargo.package.is_some() {
        let _ = rel_dirs.insert(String::new());
    }

    let mut records = Vec::new();
    for rel_dir in rel_dirs {
        if rel_dir.contains("tests/fixtures/") {
            continue;
        }
        let cargo_rel_path = CrawlView::join_rel(&rel_dir, "Cargo.toml");
        let entry = view
            .entry(&cargo_rel_path)
            .ok_or_else(|| G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from(cargo_rel_path.clone()),
                reason: "workspace member pattern did not resolve to a Cargo.toml".to_owned(),
            })?;
        if !entry.readable {
            return Err(G3RsApparchIngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let cargo = cargo_toml_parser::from_path(&entry.path.abs_path).map_err(|error| {
            G3RsApparchIngestionError::ParseFailed {
                path: entry.path.abs_path.clone(),
                reason: error.to_string(),
            }
        })?;
        let crate_name = cargo
            .package
            .as_ref()
            .and_then(|package| package.name.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                if rel_dir.is_empty() {
                    "root".to_owned()
                } else {
                    rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()
                }
            });

        records.push(CrateRecord {
            krate: G3RsApparchCrate {
                crate_name,
                cargo_rel_path,
                rel_dir: rel_dir.clone(),
                layer: layer_from_path(&rel_dir),
            },
            cargo,
        });
    }

    records.sort_by(|left, right| left.krate.cargo_rel_path.cmp(&right.krate.cargo_rel_path));
    Ok(records)
}

fn resolve_member_dirs(
    view: &CrawlView<'_>,
    cargo: &CargoToml,
) -> Result<BTreeSet<String>, G3RsApparchIngestionError> {
    let Some(workspace) = &cargo.workspace else {
        return Ok(BTreeSet::new());
    };

    let exclude_patterns = workspace
        .exclude
        .iter()
        .map(|pattern| {
            let pattern = normalize_member_pattern(pattern);
            Pattern::new(&pattern).map_err(|error| G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {error}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut resolved = BTreeSet::new();
    for member in &workspace.members {
        for rel_dir in resolve_member_pattern(view, member)? {
            if !exclude_patterns.iter().any(|pattern| pattern.matches(&rel_dir)) {
                let _ = resolved.insert(rel_dir);
            }
        }
    }

    Ok(resolved)
}

fn resolve_member_pattern(
    view: &CrawlView<'_>,
    member: &str,
) -> Result<Vec<String>, G3RsApparchIngestionError> {
    let member = normalize_member_pattern(member);
    if member.is_empty() {
        return Ok(view
            .file_exists("Cargo.toml")
            .then_some(String::new())
            .into_iter()
            .collect());
    }

    let has_glob = member.contains('*') || member.contains('?') || member.contains('[');
    if has_glob {
        let glob = Pattern::new(&member).map_err(|error| G3RsApparchIngestionError::NormalizationFailed {
            path: std::path::PathBuf::from("Cargo.toml"),
            reason: format!("invalid workspace member pattern `{member}`: {error}"),
        })?;
        let matches = view
            .all_dir_rels()
            .filter(|rel_dir| glob.matches(rel_dir))
            .filter(|rel_dir| view.file_exists(&CrawlView::join_rel(rel_dir, "Cargo.toml")))
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from("Cargo.toml"),
                reason: format!("workspace member pattern `{member}` did not resolve to any Cargo.toml"),
            });
        }
        return Ok(matches);
    }

    if view.file_exists(&CrawlView::join_rel(&member, "Cargo.toml")) {
        return Ok(vec![member]);
    }

    Err(G3RsApparchIngestionError::NormalizationFailed {
        path: std::path::PathBuf::from("Cargo.toml"),
        reason: format!("workspace member `{member}` did not resolve to a Cargo.toml"),
    })
}

fn normalize_member_pattern(member: &str) -> String {
    match member {
        "." | "./" => String::new(),
        _ => member.strip_prefix("./").unwrap_or(member).to_owned(),
    }
}

pub(super) fn layer_from_path(path: &str) -> Option<G3RsApparchLayer> {
    if contains_segment(path, "types") {
        Some(G3RsApparchLayer::Types)
    } else if contains_segment(path, "logic") {
        Some(G3RsApparchLayer::Logic)
    } else if contains_segment_pair(path, "io", "inbound") {
        Some(G3RsApparchLayer::IoInbound)
    } else if contains_segment_pair(path, "io", "outbound") {
        Some(G3RsApparchLayer::IoOutbound)
    } else {
        None
    }
}

fn contains_segment(path: &str, segment: &str) -> bool {
    path.split('/').any(|part| part == segment)
}

fn contains_segment_pair(path: &str, first: &str, second: &str) -> bool {
    let parts = path.split('/').collect::<Vec<_>>();
    parts.windows(2).any(|window| window[0] == first && window[1] == second)
}
