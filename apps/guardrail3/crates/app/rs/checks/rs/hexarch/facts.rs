use std::collections::BTreeSet;

use crate::domain::project_tree::{DirEntry, ProjectTree};

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
}

#[derive(Debug, Clone)]
pub struct ContainerFacts {
    pub app_name: String,
    pub rel_path: String,
    pub label: String,
    pub dirs: Vec<String>,
    pub files: Vec<String>,
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
    pub workspace_members: Vec<String>,
    pub discovered_crate_dirs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RootWorkspaceFacts {
    pub cargo_parse_error: Option<String>,
    pub workspace_members: Vec<String>,
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

pub fn collect(tree: &ProjectTree) -> HexarchFacts {
    let mut facts = HexarchFacts::default();
    let app_roots = rust_app_roots(tree);

    for (app_name, app_rel_dir) in &app_roots {
        let cargo_rel_path = ProjectTree::join_rel(app_rel_dir, "Cargo.toml");
        let cargo_snapshot = parse_cargo_value(tree, &cargo_rel_path);
        let workspace_members = cargo_snapshot
            .value
            .as_ref()
            .and_then(parse_workspace_members)
            .unwrap_or_default();

        let top_level_crates_rel = ProjectTree::join_rel(app_rel_dir, "crates");
        let top_level_crates_entry_count = tree
            .dir_contents(&top_level_crates_rel)
            .map(|entry| entry.dirs.len() + entry.files.len())
            .unwrap_or(0);

        let src_rel = ProjectTree::join_rel(app_rel_dir, "src");
        let discovered_crate_dirs = discover_crate_dirs(tree, app_rel_dir);

        facts.apps.push(HexAppFacts {
            app_name: app_name.clone(),
            app_rel_dir: app_rel_dir.clone(),
            cargo_rel_path: cargo_rel_path.clone(),
            cargo_parse_error: cargo_snapshot.parse_error.clone(),
            is_workspace: cargo_snapshot
                .value
                .as_ref()
                .is_some_and(|value| value.get("workspace").is_some()),
            top_level_crates_entry_count,
            src_dir_exists: tree.dir_exists(&src_rel),
        });

        facts.workspace_coverage.push(WorkspaceCoverageFacts {
            app_name: app_name.clone(),
            app_rel_dir: app_rel_dir.clone(),
            cargo_parse_error: cargo_snapshot.parse_error,
            workspace_members,
            discovered_crate_dirs,
        });

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
    }

    let root_snapshot = parse_cargo_value(tree, "Cargo.toml");
    facts.root_workspace = RootWorkspaceFacts {
        cargo_parse_error: root_snapshot.parse_error,
        workspace_members: root_snapshot
            .value
            .as_ref()
            .and_then(parse_workspace_members)
            .unwrap_or_default(),
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

fn rust_app_roots(tree: &ProjectTree) -> Vec<(String, String)> {
    let Some(apps_entry) = tree.dir_contents("apps") else {
        return Vec::new();
    };

    let mut roots = Vec::new();
    for app_name in &apps_entry.dirs {
        let app_rel_dir = ProjectTree::join_rel("apps", app_name);
        let cargo_rel = ProjectTree::join_rel(&app_rel_dir, "Cargo.toml");
        if tree.file_exists(&cargo_rel) {
            roots.push((app_name.clone(), app_rel_dir));
        }
    }
    roots.sort();
    roots
}

fn parse_workspace_members(parsed: &toml::Value) -> Option<Vec<String>> {
    parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
}

fn discover_crate_dirs(tree: &ProjectTree, app_rel_dir: &str) -> Vec<String> {
    let mut dirs = BTreeSet::new();
    for (dir_rel, entry) in &tree.structure {
        if !dir_rel.starts_with(app_rel_dir) || !entry.has_file("Cargo.toml") {
            continue;
        }
        if dir_rel == app_rel_dir {
            continue;
        }
        let rel_to_app = dir_rel
            .strip_prefix(app_rel_dir)
            .unwrap_or(dir_rel)
            .trim_start_matches('/');
        if rel_to_app.starts_with("crates/") {
            let _ = dirs.insert(rel_to_app.to_owned());
        }
    }
    dirs.into_iter().collect()
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
        let snapshot = dir_snapshot(tree, &rel_path);
        directional_containers.push(DirectionalContainerFacts {
            app_name: app_name.to_owned(),
            rel_path: rel_path.clone(),
            label: relative_hex_label(app_rel_dir, &rel_path),
            dirs: snapshot.dirs,
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
            has_gitkeep: snapshot.files.iter().any(|file| file == ".gitkeep"),
            dirs: snapshot.dirs.clone(),
            files: snapshot.files.clone(),
        });

        for subdir in &snapshot.dirs {
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
