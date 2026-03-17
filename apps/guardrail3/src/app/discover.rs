use crate::ports::outbound::FileSystem;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ProjectInfo {
    pub has_rust: bool,
    pub has_typescript: bool,
    pub cargo_workspace_root: Option<PathBuf>,
    pub workspace_members: Vec<String>,
    pub workspace_member_dirs: Vec<String>,
    pub package_json_path: Option<PathBuf>,
}

pub fn detect_project(fs: &dyn FileSystem, path: &Path) -> ProjectInfo {
    let mut info = ProjectInfo {
        has_rust: false,
        has_typescript: false,
        cargo_workspace_root: None,
        workspace_members: Vec::new(),
        workspace_member_dirs: Vec::new(),
        package_json_path: None,
    };

    // Check for Cargo.toml at path itself
    detect_rust(fs, path, &mut info);

    // Fallback: check crates/ for polyglot projects (e.g., graf)
    if !info.has_rust || info.workspace_members.is_empty() {
        let crates_path = path.join("crates");
        if crates_path.join("Cargo.toml").exists() {
            info.has_rust = false;
            info.cargo_workspace_root = None;
            info.workspace_members.clear();
            info.workspace_member_dirs.clear();
            detect_rust(fs, &crates_path, &mut info);
        }
    }

    // Fallback: check apps/backend/ for monorepo structure
    if !info.has_rust || info.workspace_members.is_empty() {
        let backend_path = path.join("apps").join("backend");
        if backend_path.exists() {
            info.has_rust = false;
            info.cargo_workspace_root = None;
            info.workspace_members.clear();
            info.workspace_member_dirs.clear();
            detect_rust(fs, &backend_path, &mut info);
        }
    }

    // Discover additional workspaces in apps/*/ (nested workspace pattern)
    // e.g., apps/shedul3r/ has its own [workspace] with internal crates
    discover_nested_workspaces(fs, path, &mut info);

    // Check for package.json
    detect_typescript(fs, path, &mut info);

    info
}

/// Find additional Cargo workspaces inside apps/*/ directories.
/// Merges their members into the existing ProjectInfo.
fn discover_nested_workspaces(fs: &dyn FileSystem, root: &Path, info: &mut ProjectInfo) {
    let apps_dir = root.join("apps");
    if !apps_dir.exists() {
        return;
    }

    for entry in fs.list_dir(&apps_dir) {
        if !entry.file_type().is_ok_and(|ft| ft.is_dir()) {
            continue;
        }
        let app_cargo = entry.path().join("Cargo.toml");
        let Some(content) = fs.read_file(&app_cargo) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };
        // Only process if it has [workspace] (it's its own workspace, not just a crate)
        let Some(workspace) = table.get("workspace") else {
            continue;
        };

        // Skip if this is the same workspace we already found
        if info
            .cargo_workspace_root
            .as_ref()
            .is_some_and(|r| *r == entry.path())
        {
            continue;
        }

        info.has_rust = true;

        // Set workspace root to the first one found if not already set
        if info.cargo_workspace_root.is_none() {
            info.cargo_workspace_root = Some(entry.path().to_path_buf());
        }

        let exclude_dirs = parse_workspace_excludes(workspace, &entry.path());
        let app_rel = entry
            .path()
            .strip_prefix(root)
            .unwrap_or(&entry.path())
            .display()
            .to_string();

        // Parse members but prefix their dirs with the app's relative path
        if let Some(members) = workspace.get("members").and_then(|m| m.as_array()) {
            for member in members {
                if let Some(member_str) = member.as_str() {
                    let pattern = entry.path().join(member_str);
                    let pattern_str = pattern.display().to_string();
                    if let Ok(paths) = glob::glob(&pattern_str) {
                        for member_path in paths.flatten() {
                            if !member_path.join("Cargo.toml").exists() {
                                continue;
                            }
                            if let Ok(rel) = member_path.strip_prefix(&entry.path()) {
                                let rel_str = rel.display().to_string();
                                if exclude_dirs.contains(&rel_str) {
                                    continue;
                                }
                            }
                            let crate_name = read_crate_name(fs, &member_path);
                            info.workspace_members.push(crate_name);
                            if let Ok(rel) = member_path.strip_prefix(root) {
                                info.workspace_member_dirs
                                    .push(rel.display().to_string());
                            } else {
                                let full_rel = format!(
                                    "{app_rel}/{}",
                                    member_path
                                        .strip_prefix(&entry.path())
                                        .unwrap_or(&member_path)
                                        .display()
                                );
                                info.workspace_member_dirs.push(full_rel);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::manual_let_else)] // reason: match with early return is clearer here
#[allow(clippy::string_slice)] // reason: parsing on known ASCII Cargo.toml content
#[allow(clippy::needless_collect)] // reason: collect needed for ownership
fn detect_rust(fs: &dyn FileSystem, path: &Path, info: &mut ProjectInfo) {
    let cargo_path = path.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    info.has_rust = true;

    let Some(content) = fs.read_file(&cargo_path) else {
        info.cargo_workspace_root = Some(path.to_path_buf());
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => {
            info.cargo_workspace_root = Some(path.to_path_buf());
            return;
        }
    };

    info.cargo_workspace_root = Some(path.to_path_buf());

    if let Some(workspace) = table.get("workspace") {
        let exclude_dirs = parse_workspace_excludes(workspace, path);
        parse_workspace_members(fs, workspace, path, &exclude_dirs, info);
    } else {
        // Single crate project
        let crate_name = read_crate_name(fs, path);
        info.workspace_members.push(crate_name);
        info.workspace_member_dirs.push(".".to_owned());
    }
}

fn parse_workspace_excludes(
    workspace: &toml::Value,
    path: &Path,
) -> std::collections::BTreeSet<String> {
    let mut exclude_dirs: std::collections::BTreeSet<String> =
        std::collections::BTreeSet::new();
    let Some(excludes) = workspace.get("exclude").and_then(|e| e.as_array()) else {
        return exclude_dirs;
    };

    for excl in excludes {
        if let Some(excl_str) = excl.as_str() {
            let excl_pattern = path.join(excl_str);
            let excl_pattern_str = excl_pattern.display().to_string();
            if let Ok(paths) = glob::glob(&excl_pattern_str) {
                for entry in paths.flatten() {
                    if let Ok(rel) = entry.strip_prefix(path) {
                        let _ = exclude_dirs.insert(rel.display().to_string());
                    }
                }
            }
            // Also add the literal pattern
            let _ = exclude_dirs.insert(excl_str.to_owned());
        }
    }

    exclude_dirs
}

fn parse_workspace_members(
    fs: &dyn FileSystem,
    workspace: &toml::Value,
    path: &Path,
    exclude_dirs: &std::collections::BTreeSet<String>,
    info: &mut ProjectInfo,
) {
    let Some(members) = workspace.get("members").and_then(|m| m.as_array()) else {
        return;
    };

    for member in members {
        if let Some(member_str) = member.as_str() {
            let pattern = path.join(member_str);
            let pattern_str = pattern.display().to_string();

            match glob::glob(&pattern_str) {
                Ok(paths) => {
                    for member_path in paths.flatten() {
                        if member_path.join("Cargo.toml").exists() {
                            if let Ok(rel) = member_path.strip_prefix(path) {
                                let rel_str = rel.display().to_string();
                                if exclude_dirs.contains(&rel_str) {
                                    continue;
                                }
                            }

                            let crate_name = read_crate_name(fs, &member_path);
                            info.workspace_members.push(crate_name);

                            if let Ok(rel) = member_path.strip_prefix(path) {
                                info.workspace_member_dirs.push(rel.display().to_string());
                            }
                        }
                    }
                }
                Err(_) => {
                    if !exclude_dirs.contains(member_str) {
                        let member_path = path.join(member_str);
                        if member_path.join("Cargo.toml").exists() {
                            let crate_name = read_crate_name(fs, &member_path);
                            info.workspace_members.push(crate_name);
                            info.workspace_member_dirs.push(member_str.to_owned());
                        }
                    }
                }
            }
        }
    }
}

fn read_crate_name(fs: &dyn FileSystem, path: &Path) -> String {
    let cargo_path = path.join("Cargo.toml");
    let Some(content) = fs.read_file(&cargo_path) else {
        return path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_owned();
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => {
            return path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_owned();
        }
    };

    table
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_owned()
}

fn detect_typescript(fs: &dyn FileSystem, path: &Path, info: &mut ProjectInfo) {
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        info.has_typescript = true;
        info.package_json_path = Some(pkg_json);
        return;
    }

    // Check apps/ subdirectories
    let applications_dir = path.join("apps");
    if applications_dir.exists() {
        for entry in fs.list_dir(&applications_dir) {
            let app_pkg = entry.path().join("package.json");
            if app_pkg.exists() {
                info.has_typescript = true;
                info.package_json_path = Some(app_pkg);
                return;
            }
        }
    }
}

