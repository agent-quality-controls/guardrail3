use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::RsHexarchRoute;
use guardrail3_app_rs_family_mapper::{DirEntry, RsProjectSurface as ProjectTree};

#[derive(Debug, Clone)]
pub struct HexAppFacts {
    pub(crate) app_name: String,
    pub(crate) app_rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) is_workspace: bool,
    pub(crate) top_level_crates_entry_count: usize,
    pub(crate) src_dir_exists: bool,
}

#[derive(Debug, Clone)]
pub struct HexRootFacts {
    pub(crate) app_name: String,
    pub(crate) app_rel_dir: String,
    pub(crate) crates_rel_dir: String,
    pub(crate) dirs: Vec<String>,
    pub(crate) files: Vec<String>,
    pub(crate) symlink_dirs: Vec<String>,
    pub(crate) symlink_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DirectionalContainerFacts {
    pub(crate) app_name: String,
    pub(crate) rel_path: String,
    pub(crate) label: String,
    pub(crate) dirs: Vec<String>,
    pub(crate) symlink_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContainerFacts {
    pub(crate) app_name: String,
    pub(crate) rel_path: String,
    pub(crate) label: String,
    pub(crate) dirs: Vec<String>,
    pub(crate) symlink_dirs: Vec<String>,
    pub(crate) files: Vec<String>,
    pub(crate) symlink_files: Vec<String>,
    pub(crate) has_gitkeep: bool,
}

#[derive(Debug, Clone)]
pub struct LeafFacts {
    pub(crate) app_name: String,
    pub(crate) rel_path: String,
    pub(crate) label: String,
    pub(crate) has_cargo: bool,
    pub(crate) has_crates_dir: bool,
    pub(crate) gitkeep_only: bool,
}

#[derive(Debug, Clone)]
pub struct WorkspaceCoverageFacts {
    pub(crate) app_name: String,
    pub(crate) app_rel_dir: String,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) is_workspace: bool,
    pub(crate) workspace_members: Vec<WorkspaceMemberFact>,
    pub(crate) app_local_cargo_roots: Vec<AppLocalCargoRootFact>,
}

#[derive(Debug, Clone)]
pub struct AppLocalCargoRootFact {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) is_workspace: bool,
}

#[derive(Debug, Clone)]
pub struct WorkspaceMemberFact {
    pub(crate) raw: String,
    pub(crate) resolved_dirs: Vec<String>,
    pub(crate) within_app_boundary: bool,
}

#[derive(Debug, Clone)]
pub struct RootWorkspaceMemberFact {
    pub(crate) raw: String,
    pub(crate) resolved_dirs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RootWorkspaceFacts {
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) workspace_members: Vec<RootWorkspaceMemberFact>,
    pub(crate) rust_app_roots: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct HexarchFacts {
    pub(crate) apps: Vec<HexAppFacts>,
    pub(crate) hex_roots: Vec<HexRootFacts>,
    pub(crate) directional_containers: Vec<DirectionalContainerFacts>,
    pub(crate) containers: Vec<ContainerFacts>,
    pub(crate) leaves: Vec<LeafFacts>,
    pub(crate) workspace_coverage: Vec<WorkspaceCoverageFacts>,
    pub(crate) root_workspace: RootWorkspaceFacts,
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
            .map(|entry| entry.dirs().len() + entry.files().len())
            .unwrap_or(0);

        let src_rel = ProjectTree::join_rel(app_rel_dir, "src");
        let mut outputs = HexRootOutputs {
            hex_roots: &mut facts.hex_roots,
            directional_containers: &mut facts.directional_containers,
            containers: &mut facts.containers,
            leaves: &mut facts.leaves,
        };
        collect_hex_roots(
            tree,
            app_name,
            app_rel_dir,
            &top_level_crates_rel,
            &mut outputs,
        );

        let workspace_members = build_workspace_member_facts(tree, app_rel_dir, &workspace_members);
        let app_local_cargo_roots = collect_app_local_cargo_roots(tree, app_rel_dir);

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
            app_local_cargo_roots,
        });
    }

    let (root_snapshot, root_workspace_members) = match route.repo_root_cargo_rel_path() {
        Some(root_cargo_rel_path) => {
            let snapshot = parse_cargo_value(tree, root_cargo_rel_path);
            let members = snapshot
                .value
                .as_ref()
                .map_or_else(|| Ok(Vec::new()), parse_workspace_members);
            (snapshot, members)
        }
        None => (CargoSnapshot::default(), Ok(Vec::new())),
    };
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
        .roots()
        .iter()
        .filter_map(|root| {
            let app_name = root.rel_dir().strip_prefix("apps/")?;
            if app_name.contains('/') {
                return None;
            }
            Some((app_name.to_owned(), root.rel_dir().to_owned()))
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

fn collect_app_local_cargo_roots(
    tree: &ProjectTree,
    app_rel_dir: &str,
) -> Vec<AppLocalCargoRootFact> {
    let mut rel_dirs = tree
        .dirs_with_file("Cargo.toml")
        .into_iter()
        .filter(|rel_dir| {
            rel_dir.starts_with(&format!("{app_rel_dir}/"))
                && !guardrail3_app_rs_placement::is_excluded_live_root_dir(rel_dir)
        })
        .collect::<Vec<_>>();
    rel_dirs.sort();

    rel_dirs
        .into_iter()
        .map(|repo_rel_dir| {
            let cargo_rel_path = ProjectTree::join_rel(&repo_rel_dir, "Cargo.toml");
            let mut cargo_snapshot = parse_cargo_value(tree, &cargo_rel_path);
            if tree.file_exists(&cargo_rel_path) && tree.file_content(&cargo_rel_path).is_none() {
                cargo_snapshot.parse_error = Some(
                    "Failed to read live app-local Cargo.toml for workspace-boundary discovery."
                        .to_owned(),
                );
            }
            let rel_dir = repo_rel_dir
                .strip_prefix(app_rel_dir)
                .unwrap_or(&repo_rel_dir)
                .trim_start_matches('/')
                .to_owned();

            AppLocalCargoRootFact {
                rel_dir,
                cargo_rel_path,
                cargo_parse_error: cargo_snapshot.parse_error,
                is_workspace: cargo_snapshot
                    .value
                    .as_ref()
                    .is_some_and(|value| value.get("workspace").is_some()),
            }
        })
        .collect()
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
        .strip_prefix(tree.root())
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

struct HexRootOutputs<'a> {
    hex_roots: &'a mut Vec<HexRootFacts>,
    directional_containers: &'a mut Vec<DirectionalContainerFacts>,
    containers: &'a mut Vec<ContainerFacts>,
    leaves: &'a mut Vec<LeafFacts>,
}

fn collect_hex_roots(
    tree: &ProjectTree,
    app_name: &str,
    app_rel_dir: &str,
    crates_rel_dir: &str,
    outputs: &mut HexRootOutputs<'_>,
) {
    let Some(crates_entry) = tree.dir_contents(crates_rel_dir) else {
        return;
    };

    outputs.hex_roots.push(HexRootFacts {
        app_name: app_name.to_owned(),
        app_rel_dir: app_rel_dir.to_owned(),
        crates_rel_dir: crates_rel_dir.to_owned(),
        dirs: crates_entry.dirs().to_vec(),
        files: crates_entry.files().to_vec(),
        symlink_dirs: crates_entry.symlink_dirs().to_vec(),
        symlink_files: crates_entry.symlink_files().to_vec(),
    });

    for group in ["adapters", "ports"] {
        let rel_path = ProjectTree::join_rel(crates_rel_dir, group);
        if !tree.dir_exists(&rel_path) {
            continue;
        }
        let snapshot = dir_snapshot(tree, &rel_path);
        outputs
            .directional_containers
            .push(DirectionalContainerFacts {
                app_name: app_name.to_owned(),
                rel_path: rel_path.clone(),
                label: relative_hex_label(app_rel_dir, &rel_path),
                dirs: snapshot.dirs().to_vec(),
                symlink_dirs: snapshot.symlink_dirs().to_vec(),
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
        outputs.containers.push(ContainerFacts {
            app_name: app_name.to_owned(),
            rel_path: rel_path.clone(),
            label: relative_hex_label(app_rel_dir, &rel_path),
            has_gitkeep: snapshot.files().iter().any(|file| file == ".gitkeep")
                && !snapshot
                    .symlink_files()
                    .iter()
                    .any(|file| file == ".gitkeep"),
            dirs: snapshot.dirs().to_vec(),
            symlink_dirs: snapshot.symlink_dirs().to_vec(),
            files: snapshot.files().to_vec(),
            symlink_files: snapshot.symlink_files().to_vec(),
        });

        for subdir in snapshot.dirs() {
            if snapshot.symlink_dirs().iter().any(|dir| dir == subdir) {
                continue;
            }
            let leaf_rel = ProjectTree::join_rel(&rel_path, subdir);
            let leaf_snapshot = dir_snapshot(tree, &leaf_rel);
            let has_cargo = leaf_snapshot
                .files()
                .iter()
                .any(|file| file == "Cargo.toml");
            let has_crates_dir = leaf_snapshot.dirs().iter().any(|dir| dir == "crates");
            let gitkeep_only = leaf_snapshot.dirs().is_empty()
                && leaf_snapshot.files().len() == 1
                && leaf_snapshot.files()[0] == ".gitkeep";

            outputs.leaves.push(LeafFacts {
                app_name: app_name.to_owned(),
                rel_path: leaf_rel.clone(),
                label: relative_hex_label(app_rel_dir, &leaf_rel),
                has_cargo,
                has_crates_dir,
                gitkeep_only,
            });

            if has_crates_dir && !has_cargo {
                let nested_crates_rel = ProjectTree::join_rel(&leaf_rel, "crates");
                collect_hex_roots(tree, app_name, app_rel_dir, &nested_crates_rel, outputs);
            }
        }
    }
}

fn dir_snapshot(tree: &ProjectTree, rel_path: &str) -> DirEntry {
    tree.dir_contents(rel_path)
        .cloned()
        .unwrap_or_else(|| DirEntry::new(Vec::new(), Vec::new(), Vec::new(), Vec::new()))
}

fn relative_hex_label(app_rel_dir: &str, rel_path: &str) -> String {
    rel_path
        .strip_prefix(app_rel_dir)
        .unwrap_or(rel_path)
        .trim_start_matches('/')
        .to_owned()
}
