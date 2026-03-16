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

    // If not found, or workspace has zero members (marker Cargo.toml for rust-analyzer),
    // check apps/backend/ (monorepo structure)
    if !info.has_rust || info.workspace_members.is_empty() {
        let backend_path = path.join("apps").join("backend");
        if backend_path.exists() {
            // Reset rust state if we're falling through from an empty marker workspace
            info.has_rust = false;
            info.cargo_workspace_root = None;
            info.workspace_members.clear();
            info.workspace_member_dirs.clear();
            detect_rust(fs, &backend_path, &mut info);
        }
    }

    // Check for package.json
    detect_typescript(fs, path, &mut info);

    info
}

#[allow(clippy::too_many_lines)] // reason: complex workspace detection logic
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

    // Both branches set cargo_workspace_root
    info.cargo_workspace_root = Some(path.to_path_buf());

    // Check for [workspace] section
    if let Some(workspace) = table.get("workspace") {
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
                                let _ = exclude_dirs.insert(rel.display().to_string());
                            }
                        }
                    }
                    // Also add the literal pattern
                    let _ = exclude_dirs.insert(excl_str.to_owned());
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
                            for member_path in paths.flatten() {
                                if member_path.join("Cargo.toml").exists() {
                                    // Check if excluded
                                    if let Ok(rel) = member_path.strip_prefix(path) {
                                        let rel_str = rel.display().to_string();
                                        if exclude_dirs.contains(&rel_str) {
                                            continue;
                                        }
                                    }

                                    // Get crate name from Cargo.toml
                                    let crate_name = read_crate_name(fs, &member_path);
                                    info.workspace_members.push(crate_name);

                                    // Store relative dir
                                    if let Ok(rel) = member_path.strip_prefix(path) {
                                        info.workspace_member_dirs.push(rel.display().to_string());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Not a glob pattern, treat as literal
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
    } else {
        // Single crate project
        let crate_name = read_crate_name(fs, path);
        info.workspace_members.push(crate_name);
        info.workspace_member_dirs.push(".".to_owned());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as stdfs;

    // ---- Bug 8: Monorepo workspace detection ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup uses expect for clarity
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    #[allow(clippy::uninlined_format_args)] // reason: assert! macros with format args
    fn detects_workspace_in_apps_backend() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        // Create a temp directory mimicking a monorepo with marker Cargo.toml at root
        // and real workspace at apps/backend/
        let tmp = std::env::temp_dir().join("guardrail3_test_monorepo");
        let _ = stdfs::remove_dir_all(&tmp);
        let _ = stdfs::create_dir_all(tmp.join("apps/backend/crates/api/src"));

        // Marker Cargo.toml at root (no workspace members)
        let _ = stdfs::write(tmp.join("Cargo.toml"), "[workspace]\nmembers = []\n");

        // Real workspace at apps/backend/
        let _ = stdfs::write(
            tmp.join("apps/backend/Cargo.toml"),
            "[workspace]\nmembers = [\"crates/api\"]\n",
        );

        // Crate Cargo.toml
        let _ = stdfs::write(
            tmp.join("apps/backend/crates/api/Cargo.toml"),
            "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        );
        let _ = stdfs::write(
            tmp.join("apps/backend/crates/api/src/main.rs"),
            "fn main() {}\n",
        );

        // Also add a package.json so TypeScript is detected
        let _ = stdfs::write(tmp.join("package.json"), "{}");

        let project = detect_project(&fs, &tmp);

        assert!(project.has_rust, "Should detect Rust");
        assert!(project.has_typescript, "Should detect TypeScript");

        let workspace_root = project
            .cargo_workspace_root
            .as_ref()
            .expect("Should have workspace root");
        assert!(
            workspace_root.ends_with("apps/backend"),
            "Workspace root should be apps/backend, got {:?}",
            workspace_root
        );
        assert!(
            !project.workspace_members.is_empty(),
            "Should have detected workspace members"
        );
        assert!(
            project.workspace_members.contains(&"api".to_owned()),
            "Should find 'api' crate, got {:?}",
            project.workspace_members
        );

        // Cleanup
        let _ = stdfs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup uses expect for clarity
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn direct_workspace_detected_at_root() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_test_direct_ws");
        let _ = stdfs::remove_dir_all(&tmp);
        let _ = stdfs::create_dir_all(tmp.join("crates/core/src"));

        let _ = stdfs::write(
            tmp.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/core\"]\n",
        );
        let _ = stdfs::write(
            tmp.join("crates/core/Cargo.toml"),
            "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        );
        let _ = stdfs::write(tmp.join("crates/core/src/lib.rs"), "");

        let project = detect_project(&fs, &tmp);
        assert!(project.has_rust);

        let workspace_root = project
            .cargo_workspace_root
            .as_ref()
            .expect("Should have workspace root");
        // Should be the root itself, not apps/backend
        assert_eq!(
            workspace_root.as_path(),
            tmp.as_path(),
            "Direct workspace should have root as workspace root"
        );

        let _ = stdfs::remove_dir_all(&tmp);
    }
}
