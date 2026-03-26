use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::RsHexarchRoute;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

#[derive(Debug, Clone)]
pub struct HexAppFacts {
    pub app_name: String,
    pub app_rel_dir: String,
    pub cargo_rel_path: String,
    pub cargo_parse_error: Option<String>,
    pub is_workspace: bool,
    pub top_level_crates_entry_count: usize,
    pub src_dir_exists: bool,
}

#[derive(Debug, Clone)]
pub struct HexRootFacts {
    pub app_name: String,
    pub app_rel_dir: String,
    pub crates_rel_dir: String,
    pub dirs: Vec<String>,
    pub files: Vec<String>,
    pub symlink_dirs: Vec<String>,
    pub symlink_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DirectionalContainerFacts {
    pub app_name: String,
    pub rel_path: String,
    pub label: String,
    pub dirs: Vec<String>,
    pub symlink_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContainerFacts {
    pub app_name: String,
    pub rel_path: String,
    pub label: String,
    pub dirs: Vec<String>,
    pub symlink_dirs: Vec<String>,
    pub files: Vec<String>,
    pub symlink_files: Vec<String>,
    pub has_gitkeep: bool,
}

#[derive(Debug, Clone)]
pub struct LeafFacts {
    pub app_name: String,
    pub rel_path: String,
    pub label: String,
    pub has_cargo: bool,
    pub has_crates_dir: bool,
    pub gitkeep_only: bool,
}

#[derive(Debug, Clone)]
pub struct WorkspaceCoverageFacts {
    pub app_name: String,
    pub app_rel_dir: String,
    pub cargo_parse_error: Option<String>,
    pub is_workspace: bool,
    pub workspace_members: Vec<WorkspaceMemberFact>,
    pub discovered_crate_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceMemberFact {
    pub raw: String,
    pub resolved_dirs: Vec<String>,
    pub within_app_boundary: bool,
}

#[derive(Debug, Clone)]
pub struct RootWorkspaceMemberFact {
    pub raw: String,
    pub resolved_dirs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RootWorkspaceFacts {
    pub cargo_parse_error: Option<String>,
    pub workspace_members: Vec<RootWorkspaceMemberFact>,
    pub rust_app_roots: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct HexarchFacts {
    pub apps: Vec<HexAppFacts>,
    pub hex_roots: Vec<HexRootFacts>,
    pub directional_containers: Vec<DirectionalContainerFacts>,
    pub containers: Vec<ContainerFacts>,
    pub leaves: Vec<LeafFacts>,
    pub workspace_coverage: Vec<WorkspaceCoverageFacts>,
    pub root_workspace: RootWorkspaceFacts,
}

pub fn collect(tree: &ProjectTree, route: &RsHexarchRoute) -> HexarchFacts {
    let mut facts = HexarchFacts::default();
    let app_roots = routed_app_roots(route);

    for (app_name, app_rel_dir) in &app_roots {
        let cargo_rel_path = ProjectTree::join_rel(app_rel_dir, "Cargo.toml");
        let cargo_snapshot = parse_cargo_value(tree, &cargo_rel_path);
        let workspace_members = cargo_snapshot
            .value
            .as_ref()
            .map_or_else(|| Ok(Vec::new()), parse_workspace_members);
        let is_workspace = cargo_snapshot
            .value
            .as_ref()
            .is_some_and(|value| value.get("workspace").is_some());
        let workspace_members_error = workspace_members
            .as_ref()
            .err()
            .map(std::string::ToString::to_string);
        let workspace_members = workspace_members.unwrap_or_default();

        let top_level_crates_rel = ProjectTree::join_rel(app_rel_dir, "crates");
        let top_level_crates_entry_count = tree
            .dir_contents(&top_level_crates_rel)
            .map(|entry| entry.dirs.len() + entry.files.len())
            .unwrap_or(0);

        let src_rel = ProjectTree::join_rel(app_rel_dir, "src");
        let leaves_start = facts.leaves.len();

        collect_hex_roots(
            tree,
            app_name,
            app_rel_dir,
            &top_level_crates_rel,
            &mut facts.hex_roots,
            &mut facts.directional_containers,
            &mut facts.containers,
            &mut facts.leaves,
        );

        let discovered_crate_dirs =
            discovered_workspace_crate_dirs(&facts.leaves[leaves_start..], app_rel_dir);
        let workspace_members = build_workspace_member_facts(tree, app_rel_dir, &workspace_members);

        facts.apps.push(HexAppFacts {
            app_name: app_name.clone(),
            app_rel_dir: app_rel_dir.clone(),
            cargo_rel_path: cargo_rel_path.clone(),
            cargo_parse_error: cargo_snapshot
                .parse_error
                .clone()
                .or(workspace_members_error.clone()),
            is_workspace,
            top_level_crates_entry_count,
            src_dir_exists: tree.dir_exists(&src_rel),
        });

        facts.workspace_coverage.push(WorkspaceCoverageFacts {
            app_name: app_name.clone(),
            app_rel_dir: app_rel_dir.clone(),
            cargo_parse_error: cargo_snapshot.parse_error.or(workspace_members_error),
            is_workspace,
            workspace_members,
            discovered_crate_dirs,
        });
    }

    let root_snapshot = parse_cargo_value(tree, "Cargo.toml");
    let root_workspace_members = root_snapshot
        .value
        .as_ref()
        .map_or_else(|| Ok(Vec::new()), parse_workspace_members);
    facts.root_workspace = RootWorkspaceFacts {
        cargo_parse_error: root_snapshot.parse_error.clone().or_else(|| {
            root_workspace_members
                .as_ref()
                .err()
                .map(std::string::ToString::to_string)
        }),
        workspace_members: build_root_workspace_member_facts(
            tree,
            &root_workspace_members.unwrap_or_default(),
        ),
        rust_app_roots: app_roots.into_iter().map(|(_, rel)| rel).collect(),
    };

    facts
}

#[derive(Debug, Clone, Default)]
struct CargoSnapshot {
    value: Option<toml::Value>,
    parse_error: Option<String>,
}

fn parse_cargo_value(tree: &ProjectTree, rel_path: &str) -> CargoSnapshot {
    let Some(content) = tree.file_content(rel_path) else {
        return CargoSnapshot::default();
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(value) => CargoSnapshot {
            value: Some(value),
            parse_error: None,
        },
        Err(parse_error) => CargoSnapshot {
            value: None,
            parse_error: Some(parse_error.to_string()),
        },
    }
}

fn routed_app_roots(route: &RsHexarchRoute) -> Vec<(String, String)> {
    let mut roots = route
        .roots
        .iter()
        .filter_map(|root| {
            let app_name = root.rel_dir.strip_prefix("apps/")?;
            if app_name.contains('/') {
                return None;
            }
            Some((app_name.to_owned(), root.rel_dir.clone()))
        })
        .collect::<Vec<_>>();
    roots.sort();
    roots
}

fn parse_workspace_members(parsed: &toml::Value) -> Result<Vec<String>, String> {
    let Some(workspace) = parsed.get("workspace") else {
        return Ok(Vec::new());
    };
    let Some(members) = workspace.get("members") else {
        return Ok(Vec::new());
    };
    let Some(members) = members.as_array() else {
        return Err("[workspace].members must be an array of strings".to_owned());
    };

    members
        .iter()
        .enumerate()
        .map(|(index, member)| {
            member.as_str().map(str::to_owned).ok_or_else(|| {
                format!(
                    "[workspace].members[{index}] must be a string, got {}",
                    toml_value_kind(member)
                )
            })
        })
        .collect()
}

fn toml_value_kind(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "boolean",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}

fn discovered_workspace_crate_dirs(leaves: &[LeafFacts], app_rel_dir: &str) -> Vec<String> {
    let mut dirs = BTreeSet::new();
    for leaf in leaves {
        if !leaf.has_cargo || leaf.has_crates_dir {
            continue;
        }
        let rel_to_app = leaf
            .rel_path
            .strip_prefix(app_rel_dir)
            .unwrap_or(&leaf.rel_path)
            .trim_start_matches('/');
        let _ = dirs.insert(rel_to_app.to_owned());
    }
    dirs.into_iter().collect()
}

fn build_workspace_member_facts(
    tree: &ProjectTree,
    app_rel_dir: &str,
    workspace_members: &[String],
) -> Vec<WorkspaceMemberFact> {
    let mut facts = Vec::new();
    for member in workspace_members {
        let mut resolved = BTreeSet::new();
        let absolute_member = member.starts_with('/');
        let repo_pattern = if absolute_member {
            member.trim_end_matches('/').to_owned()
        } else {
            normalize_member_pattern_against_base(app_rel_dir, member)
        };
        let within_app_boundary =
            !absolute_member && is_within_app_boundary(app_rel_dir, &repo_pattern);
        if !absolute_member {
            if looks_like_glob(member) {
                for match_rel in tree.matching_dir_rels(&repo_pattern) {
                    insert_app_relative_member(&mut resolved, app_rel_dir, &match_rel);
                }
            } else {
                let normalized = super::dependency_facts::normalize_path(app_rel_dir, member);
                insert_app_relative_member(&mut resolved, app_rel_dir, &normalized);
            }
        }
        facts.push(WorkspaceMemberFact {
            raw: member.clone(),
            resolved_dirs: resolved.into_iter().collect(),
            within_app_boundary,
        });
    }
    facts
}

fn build_root_workspace_member_facts(
    tree: &ProjectTree,
    workspace_members: &[String],
) -> Vec<RootWorkspaceMemberFact> {
    let mut facts = Vec::new();
    for member in workspace_members {
        let mut resolved = BTreeSet::new();
        if member.starts_with('/') {
            if let Some(repo_rel) = absolute_member_to_repo_rel(tree, member) {
                insert_root_workspace_member(&mut resolved, &repo_rel);
            }
        } else if looks_like_glob(member) {
            let pattern = normalize_member_pattern_against_base("", member);
            for match_rel in tree.matching_dir_rels(&pattern) {
                insert_root_workspace_member(&mut resolved, &match_rel);
            }
        } else {
            let normalized = super::dependency_facts::normalize_path("", member);
            insert_root_workspace_member(&mut resolved, &normalized);
        }
        facts.push(RootWorkspaceMemberFact {
            raw: member.clone(),
            resolved_dirs: resolved.into_iter().collect(),
        });
    }
    facts
}

fn insert_app_relative_member(resolved: &mut BTreeSet<String>, app_rel_dir: &str, repo_rel: &str) {
    let rel_to_app = repo_rel
        .strip_prefix(app_rel_dir)
        .unwrap_or(repo_rel)
        .trim_start_matches('/');
    let _ = resolved.insert(rel_to_app.to_owned());
}

fn insert_root_workspace_member(resolved: &mut BTreeSet<String>, repo_rel: &str) {
    let rel = repo_rel.trim_matches('/');
    let _ = resolved.insert(rel.to_owned());
}

fn absolute_member_to_repo_rel(tree: &ProjectTree, member: &str) -> Option<String> {
    let absolute = std::path::Path::new(member);
    absolute
        .strip_prefix(&tree.root)
        .ok()
        .map(|rel| rel.to_string_lossy().replace('\\', "/"))
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn normalize_member_pattern_against_base(base: &str, pattern: &str) -> String {
    let mut parts = if base.is_empty() {
        Vec::new()
    } else {
        base.split('/').collect::<Vec<_>>()
    };
    for segment in pattern.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                let _ = parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}

fn is_within_app_boundary(app_rel_dir: &str, repo_pattern: &str) -> bool {
    let crates_root = ProjectTree::join_rel(app_rel_dir, "crates");
    repo_pattern == crates_root || repo_pattern.starts_with(&format!("{crates_root}/"))
}

#[allow(clippy::too_many_arguments)] // reason: recursive collector threads family-local outputs
fn collect_hex_roots(
    tree: &ProjectTree,
    app_name: &str,
    app_rel_dir: &str,
    crates_rel_dir: &str,
    hex_roots: &mut Vec<HexRootFacts>,
    directional_containers: &mut Vec<DirectionalContainerFacts>,
    containers: &mut Vec<ContainerFacts>,
    leaves: &mut Vec<LeafFacts>,
) {
    let Some(crates_entry) = tree.dir_contents(crates_rel_dir) else {
        return;
    };

    hex_roots.push(HexRootFacts {
        app_name: app_name.to_owned(),
        app_rel_dir: app_rel_dir.to_owned(),
        crates_rel_dir: crates_rel_dir.to_owned(),
        dirs: crates_entry.dirs.clone(),
        files: crates_entry.files.clone(),
        symlink_dirs: crates_entry.symlink_dirs.clone(),
        symlink_files: crates_entry.symlink_files.clone(),
    });

    for group in ["adapters", "ports"] {
        let rel_path = ProjectTree::join_rel(crates_rel_dir, group);
        if !tree.dir_exists(&rel_path) {
            continue;
        }
        let snapshot = dir_snapshot(tree, &rel_path);
        directional_containers.push(DirectionalContainerFacts {
            app_name: app_name.to_owned(),
            rel_path: rel_path.clone(),
            label: relative_hex_label(app_rel_dir, &rel_path),
            dirs: snapshot.dirs,
            symlink_dirs: snapshot.symlink_dirs,
        });
    }

    for suffix in [
        "app",
        "domain",
        "adapters/inbound",
        "adapters/outbound",
        "ports/inbound",
        "ports/outbound",
    ] {
        let rel_path = ProjectTree::join_rel(crates_rel_dir, suffix);
        let snapshot = dir_snapshot(tree, &rel_path);
        containers.push(ContainerFacts {
            app_name: app_name.to_owned(),
            rel_path: rel_path.clone(),
            label: relative_hex_label(app_rel_dir, &rel_path),
            has_gitkeep: snapshot.files.iter().any(|file| file == ".gitkeep")
                && !snapshot.symlink_files.iter().any(|file| file == ".gitkeep"),
            dirs: snapshot.dirs.clone(),
            symlink_dirs: snapshot.symlink_dirs.clone(),
            files: snapshot.files.clone(),
            symlink_files: snapshot.symlink_files.clone(),
        });

        for subdir in &snapshot.dirs {
            if snapshot.symlink_dirs.iter().any(|dir| dir == subdir) {
                continue;
            }
            let leaf_rel = ProjectTree::join_rel(&rel_path, subdir);
            let leaf_snapshot = dir_snapshot(tree, &leaf_rel);
            let has_cargo = leaf_snapshot.files.iter().any(|file| file == "Cargo.toml");
            let has_crates_dir = leaf_snapshot.dirs.iter().any(|dir| dir == "crates");
            let gitkeep_only = leaf_snapshot.dirs.is_empty()
                && leaf_snapshot.files.len() == 1
                && leaf_snapshot.files[0] == ".gitkeep";

            leaves.push(LeafFacts {
                app_name: app_name.to_owned(),
                rel_path: leaf_rel.clone(),
                label: relative_hex_label(app_rel_dir, &leaf_rel),
                has_cargo,
                has_crates_dir,
                gitkeep_only,
            });

            if has_crates_dir && !has_cargo {
                let nested_crates_rel = ProjectTree::join_rel(&leaf_rel, "crates");
                collect_hex_roots(
                    tree,
                    app_name,
                    app_rel_dir,
                    &nested_crates_rel,
                    hex_roots,
                    directional_containers,
                    containers,
                    leaves,
                );
            }
        }
    }
}

fn dir_snapshot(tree: &ProjectTree, rel_path: &str) -> DirEntry {
    tree.dir_contents(rel_path).cloned().unwrap_or(DirEntry {
        dirs: Vec::new(),
        files: Vec::new(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    })
}

fn relative_hex_label(app_rel_dir: &str, rel_path: &str) -> String {
    rel_path
        .strip_prefix(app_rel_dir)
        .unwrap_or(rel_path)
        .trim_start_matches('/')
        .to_owned()
}
