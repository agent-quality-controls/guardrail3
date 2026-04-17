use std::collections::BTreeSet;

use cargo_toml_parser::{types::CargoToml, parse};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};
use glob::Pattern;

use crate::ingest::IngestionError;

#[derive(Debug, Clone)]
pub(crate) struct OwnedTestRoot {
    pub(crate) root_rel_dir: String,
    pub(crate) runtime_rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo: CargoToml,
    pub(crate) root_manifest: Option<CargoToml>,
}

#[derive(Debug, Clone)]
pub(crate) struct TestRootDiscovery {
    pub(crate) roots: Vec<OwnedTestRoot>,
    pub(crate) workspace_manifest: Option<CargoToml>,
    pub(crate) workspace_members: BTreeSet<String>,
}

pub(crate) fn discover(crawl: &G3RsWorkspaceCrawl) -> Result<TestRootDiscovery, IngestionError> {
    let Some(root_entry) = g3rs_workspace_crawl::root_file(crawl, "Cargo.toml") else {
        return Ok(TestRootDiscovery {
            roots: Vec::new(),
            workspace_manifest: None,
            workspace_members: BTreeSet::new(),
        });
    };

    let root_manifest = parse_required_manifest(crawl, "Cargo.toml")?;
    let workspace_members = if let Some(workspace) = root_manifest.workspace.as_ref() {
        select_workspace_member_dirs(
            crawl,
            workspace.members.as_slice(),
            workspace.exclude.as_slice(),
        )?
    } else {
        BTreeSet::new()
    };

    let mut owned_root_dirs = BTreeSet::from([String::new()]);
    owned_root_dirs.extend(
        workspace_members
            .iter()
            .map(|member_dir| component_container_root(member_dir, crawl))
            .collect::<BTreeSet<_>>(),
    );

    let mut roots = owned_root_dirs
        .into_iter()
        .filter_map(|root_rel_dir| build_owned_root(crawl, &root_rel_dir).transpose())
        .collect::<Result<Vec<_>, _>>()?;
    roots.sort_by(|left, right| left.root_rel_dir.cmp(&right.root_rel_dir));

    let workspace_manifest = root_manifest.workspace.is_some().then_some(root_manifest);
    let _ = root_entry;
    Ok(TestRootDiscovery {
        roots,
        workspace_manifest,
        workspace_members,
    })
}

fn build_owned_root(
    crawl: &G3RsWorkspaceCrawl,
    root_rel_dir: &str,
) -> Result<Option<OwnedTestRoot>, IngestionError> {
    let runtime_cargo_rel_path = if g3rs_workspace_crawl::entry(
        crawl,
        &join_under_root(root_rel_dir, "crates/runtime/Cargo.toml"),
    )
    .is_some()
    {
        join_under_root(root_rel_dir, "crates/runtime/Cargo.toml")
    } else {
        let root_cargo_rel_path = join_under_root(root_rel_dir, "Cargo.toml");
        let Some(root_entry) = g3rs_workspace_crawl::entry(crawl, &root_cargo_rel_path) else {
            return Ok(None);
        };
        let root_manifest = parse_required_manifest(crawl, &root_cargo_rel_path)?;
        if root_manifest.package.is_none() {
            let _ = root_entry;
            return Ok(None);
        }
        root_cargo_rel_path
    };

    let cargo = parse_required_manifest(crawl, &runtime_cargo_rel_path)?;
    let runtime_rel_dir = parent_dir(&runtime_cargo_rel_path).to_owned();
    let root_cargo_rel_path = join_under_root(root_rel_dir, "Cargo.toml");
    let root_manifest = if root_cargo_rel_path == runtime_cargo_rel_path {
        Some(cargo.clone())
    } else if g3rs_workspace_crawl::entry(crawl, &root_cargo_rel_path).is_some() {
        Some(parse_required_manifest(crawl, &root_cargo_rel_path)?)
    } else {
        None
    };

    Ok(Some(OwnedTestRoot {
        root_rel_dir: root_rel_dir.to_owned(),
        runtime_rel_dir,
        cargo_rel_path: runtime_cargo_rel_path,
        cargo,
        root_manifest,
    }))
}

fn component_container_root(rel_dir: &str, crawl: &G3RsWorkspaceCrawl) -> String {
    let candidate = if matches!(
        rel_dir,
        "crates/runtime"
            | "crates/assertions"
            | "crates/assertions_common"
            | "assertions"
            | "crates/test_support"
            | "test_support"
    ) {
        Some(String::new())
    } else {
        rel_dir
            .strip_suffix("/crates/runtime")
            .or_else(|| rel_dir.strip_suffix("/crates/assertions"))
            .or_else(|| rel_dir.strip_suffix("/crates/assertions_common"))
            .or_else(|| rel_dir.strip_suffix("/assertions"))
            .or_else(|| rel_dir.strip_suffix("/crates/test_support"))
            .or_else(|| rel_dir.strip_suffix("/test_support"))
            .map(ToOwned::to_owned)
    };

    let Some(candidate) = candidate else {
        return rel_dir.to_owned();
    };
    let candidate_has_package =
        g3rs_workspace_crawl::entry(crawl, &join_under_root(&candidate, "Cargo.toml")).is_some();
    let candidate_has_runtime = g3rs_workspace_crawl::entry(
        crawl,
        &join_under_root(&candidate, "crates/runtime/Cargo.toml"),
    )
    .is_some();
    if candidate_has_package || candidate_has_runtime {
        candidate
    } else {
        rel_dir.to_owned()
    }
}

fn parse_required_manifest(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<CargoToml, IngestionError> {
    let entry = g3rs_workspace_crawl::entry(crawl, rel_path).ok_or_else(|| {
        IngestionError::ParseFailed {
            path: crawl.root_abs_path.join(rel_path),
            reason: "required Cargo.toml entry is missing from crawl".to_owned(),
        }
    })?;
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;
    parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })
}

fn select_workspace_member_dirs(
    crawl: &G3RsWorkspaceCrawl,
    members: &[String],
    exclude: &[String],
) -> Result<BTreeSet<String>, IngestionError> {
    let member_patterns = members
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let exclude_patterns = exclude
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let member_dirs = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| manifest_dir_from_manifest_path(entry.path.rel_path.as_str()))
        .filter(|member_dir| {
            !member_dir.is_empty()
                && member_patterns
                    .iter()
                    .any(|pattern| pattern.matches(member_dir))
                && !exclude_patterns
                    .iter()
                    .any(|pattern| pattern.matches(member_dir))
        })
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    for (raw_pattern, pattern) in members.iter().zip(member_patterns.iter()) {
        if !member_dirs
            .iter()
            .any(|member_dir| pattern.matches(member_dir))
        {
            return Err(IngestionError::ParseFailed {
                path: crawl.root_abs_path.join("Cargo.toml"),
                reason: format!(
                    "workspace member pattern `{raw_pattern}` did not resolve to any Cargo.toml"
                ),
            });
        }
    }

    Ok(member_dirs)
}

pub(crate) fn join_under_root(root_rel_dir: &str, child_rel: &str) -> String {
    if root_rel_dir.is_empty() {
        child_rel.to_owned()
    } else {
        format!("{root_rel_dir}/{child_rel}")
    }
}

pub(crate) fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn manifest_dir_from_manifest_path(rel_path: &str) -> Option<&str> {
    if rel_path == "Cargo.toml" {
        return Some("");
    }

    rel_path.strip_suffix("/Cargo.toml")
}
