use std::path::Path;

use crate::discover::ProjectInfo;
use crate::report::types::{CheckResult, Severity};

// R51: Dependency direction — iterate workspace member Cargo.tomls
pub fn check_all_dependency_directions(
    workspace_root: &Path,
    project: &ProjectInfo,
    results: &mut Vec<CheckResult>,
) {
    for member_dir in &project.workspace_member_dirs {
        let cargo_path = workspace_root.join(member_dir).join("Cargo.toml");
        if !cargo_path.exists() {
            continue;
        }
        check_dependency_direction(&cargo_path, member_dir, results);
    }
}

fn check_dependency_direction(cargo_path: &Path, member_dir: &str, results: &mut Vec<CheckResult>) {
    let Some(content) = crate::fs::read_file(cargo_path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // Determine crate kind from path
    let is_domain = member_dir.contains("/domain/")
        || member_dir.contains("/domain-")
        || member_dir.ends_with("/domain")
        || member_dir == "domain";
    let is_types = member_dir.contains("/types/")
        || member_dir.contains("/types-")
        || member_dir.ends_with("/types")
        || member_dir == "types";
    let is_commands = member_dir.contains("/commands/")
        || member_dir.contains("/commands-")
        || member_dir.ends_with("/commands")
        || member_dir == "commands";
    let is_repo = member_dir.contains("/repo/")
        || member_dir.contains("/repo-")
        || member_dir.ends_with("/repo")
        || member_dir == "repo"
        || member_dir.contains("/ports/")
        || member_dir.contains("/ports-")
        || member_dir.ends_with("/ports")
        || member_dir == "ports";

    if !is_domain && !is_types && !is_commands && !is_repo {
        return;
    }

    // Banned dependency names per crate kind (exact name match)
    let banned_for_domain_types: &[&str] = &[
        "db", "api", "commands", "adapters", "sqlx", "axum", "reqwest",
    ];
    let banned_for_commands: &[&str] = &["db", "api", "adapters", "sqlx", "axum", "reqwest"];
    let banned_for_repo: &[&str] = &["db", "api", "commands", "adapters", "sqlx", "axum"];

    let banned = if is_domain || is_types {
        banned_for_domain_types
    } else if is_commands {
        banned_for_commands
    } else {
        banned_for_repo
    };

    let kind = if is_domain {
        "domain"
    } else if is_types {
        "types"
    } else if is_commands {
        "commands"
    } else {
        "repo/ports"
    };

    // Suffixes that indicate architectural layer crates
    let banned_suffixes: &[&str] = &["-db", "-api", "-adapters", "-commands", "-repo", "-ports"];

    if let Some(deps) = table.get("dependencies") {
        if let Some(dep_table) = deps.as_table() {
            for dep_name in dep_table.keys() {
                // Exact crate name matching
                let exact_match = banned.contains(&dep_name.as_str());
                // Suffix matching for prefixed crate names (e.g. "myapp-db", "myapp-api")
                let suffix_match = banned_suffixes
                    .iter()
                    .any(|suffix| dep_name.ends_with(suffix));

                if exact_match || suffix_match {
                    results.push(CheckResult {
                        id: "R51".to_owned(),
                        severity: Severity::Error,
                        title: "Dependency direction violation".to_owned(),
                        message: format!("{kind} crate ({member_dir}) depends on \"{dep_name}\""),
                        file: Some(cargo_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }
}

// R52: Dependency graph inventory
pub fn check_dependency_graph(
    workspace_root: &Path,
    project: &ProjectInfo,
    results: &mut Vec<CheckResult>,
) {
    for (idx, member_dir) in project.workspace_member_dirs.iter().enumerate() {
        let cargo_path = workspace_root.join(member_dir).join("Cargo.toml");
        if !cargo_path.exists() {
            continue;
        }

        let Some(content) = crate::fs::read_file(&cargo_path) else {
            continue;
        };

        let table: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let crate_name = project
            .workspace_members
            .get(idx)
            .map_or(member_dir.as_str(), std::string::String::as_str);

        if let Some(deps) = table.get("dependencies") {
            if let Some(dep_table) = deps.as_table() {
                // Collect internal deps (path dependencies)
                let mut internal_deps = Vec::new();
                for (dep_name, dep_val) in dep_table {
                    let is_path = if let toml::Value::Table(t) = dep_val {
                        t.get("path").is_some()
                    } else {
                        false
                    };
                    if is_path {
                        internal_deps.push(dep_name.clone());
                    }
                }

                if !internal_deps.is_empty() {
                    internal_deps.sort();
                    results.push(CheckResult {
                        id: "R52".to_owned(),
                        severity: Severity::Info,
                        title: format!("{crate_name} internal deps"),
                        message: format!("depends on: {}", internal_deps.join(", ")),
                        file: Some(cargo_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }
}
