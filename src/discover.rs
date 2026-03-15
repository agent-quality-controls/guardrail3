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

pub fn detect_project(path: &Path) -> ProjectInfo {
    let mut info = ProjectInfo {
        has_rust: false,
        has_typescript: false,
        cargo_workspace_root: None,
        workspace_members: Vec::new(),
        workspace_member_dirs: Vec::new(),
        package_json_path: None,
    };

    // Check for Cargo.toml at path itself
    detect_rust(path, &mut info);

    // If not found, or workspace has zero members (marker Cargo.toml for rust-analyzer),
    // check apps/backend/ (monorepo structure)
    if !info.has_rust || (info.has_rust && info.workspace_members.is_empty()) {
        let backend_path = path.join("apps").join("backend");
        if backend_path.exists() {
            // Reset rust state if we're falling through from an empty marker workspace
            info.has_rust = false;
            info.cargo_workspace_root = None;
            info.workspace_members.clear();
            info.workspace_member_dirs.clear();
            detect_rust(&backend_path, &mut info);
        }
    }

    // Check for package.json
    detect_typescript(path, &mut info);

    info
}

fn detect_rust(path: &Path, info: &mut ProjectInfo) {
    let cargo_path = path.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    info.has_rust = true;

    let content = match std::fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => {
            info.cargo_workspace_root = Some(path.to_path_buf());
            return;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => {
            info.cargo_workspace_root = Some(path.to_path_buf());
            return;
        }
    };

    // Check for [workspace] section
    if let Some(workspace) = table.get("workspace") {
        info.cargo_workspace_root = Some(path.to_path_buf());

        // Parse workspace exclude patterns
        let mut exclude_dirs: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        if let Some(excludes) = workspace.get("exclude").and_then(|e| e.as_array()) {
            for excl in excludes {
                if let Some(excl_str) = excl.as_str() {
                    let excl_pattern = path.join(excl_str);
                    let excl_pattern_str = excl_pattern.display().to_string();
                    if let Ok(paths) = glob::glob(&excl_pattern_str) {
                        for entry in paths.flatten() {
                            if let Ok(rel) = entry.strip_prefix(path) {
                                exclude_dirs.insert(rel.display().to_string());
                            }
                        }
                    }
                    // Also add the literal pattern
                    exclude_dirs.insert(excl_str.to_string());
                }
            }
        }

        // Parse workspace members
        if let Some(members) = workspace.get("members").and_then(|m| m.as_array()) {
            for member in members {
                if let Some(member_str) = member.as_str() {
                    // Expand glob patterns
                    let pattern = path.join(member_str);
                    let pattern_str = pattern.display().to_string();

                    match glob::glob(&pattern_str) {
                        Ok(paths) => {
                            for entry in paths {
                                if let Ok(member_path) = entry {
                                    if member_path.join("Cargo.toml").exists() {
                                        // Check if excluded
                                        if let Ok(rel) = member_path.strip_prefix(path) {
                                            let rel_str = rel.display().to_string();
                                            if exclude_dirs.contains(&rel_str) {
                                                continue;
                                            }
                                        }

                                        // Get crate name from Cargo.toml
                                        let crate_name = read_crate_name(&member_path);
                                        info.workspace_members.push(crate_name);

                                        // Store relative dir
                                        if let Ok(rel) = member_path.strip_prefix(path) {
                                            info.workspace_member_dirs
                                                .push(rel.display().to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Not a glob pattern, treat as literal
                            if !exclude_dirs.contains(member_str) {
                                let member_path = path.join(member_str);
                                if member_path.join("Cargo.toml").exists() {
                                    let crate_name = read_crate_name(&member_path);
                                    info.workspace_members.push(crate_name);
                                    info.workspace_member_dirs
                                        .push(member_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Single crate project
        info.cargo_workspace_root = Some(path.to_path_buf());
        let crate_name = read_crate_name(path);
        info.workspace_members.push(crate_name);
        info.workspace_member_dirs.push(".".to_string());
    }
}

fn read_crate_name(path: &Path) -> String {
    let cargo_path = path.join("Cargo.toml");
    let content = match std::fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => {
            return path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => {
            return path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
        }
    };

    table
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_string()
}

fn detect_typescript(path: &Path, info: &mut ProjectInfo) {
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        info.has_typescript = true;
        info.package_json_path = Some(pkg_json);
        return;
    }

    // Check apps/ subdirectories
    let apps_dir = path.join("apps");
    if apps_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&apps_dir) {
            for entry in entries.flatten() {
                let app_pkg = entry.path().join("package.json");
                if app_pkg.exists() {
                    info.has_typescript = true;
                    info.package_json_path = Some(app_pkg);
                    return;
                }
            }
        }
    }
}
