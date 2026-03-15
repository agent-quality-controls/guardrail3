use std::path::Path;

use crate::report::types::{CheckResult, Severity};

// R55-R57: workspace metadata & release profile
#[allow(clippy::too_many_lines)] // reason: workspace metadata validation
pub fn check_workspace_metadata(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let cargo_path = workspace_root.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let Ok(content) = std::fs::read_to_string(&cargo_path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // R55: Report workspace edition and rust-version
    let edition = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("edition"))
        .and_then(|e| e.as_str())
        .or_else(|| {
            table
                .get("package")
                .and_then(|p| p.get("edition"))
                .and_then(|e| e.as_str())
        });

    let rust_version = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("rust-version"))
        .and_then(|r| r.as_str())
        .or_else(|| {
            table
                .get("package")
                .and_then(|p| p.get("rust-version"))
                .and_then(|r| r.as_str())
        });

    let mut meta_parts = Vec::new();
    if let Some(ed) = edition {
        meta_parts.push(format!("edition = {ed}"));
    }
    if let Some(rv) = rust_version {
        meta_parts.push(format!("rust-version = {rv}"));
    }

    if !meta_parts.is_empty() {
        results.push(CheckResult {
            id: "R55".to_owned(),
            severity: Severity::Info,
            title: "Workspace metadata".to_owned(),
            message: meta_parts.join(", "),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    }

    // R56: Report workspace publish status
    let publish = table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("publish"))
        .or_else(|| table.get("package").and_then(|p| p.get("publish")));

    if let Some(p) = publish {
        results.push(CheckResult {
            id: "R56".to_owned(),
            severity: Severity::Info,
            title: "Publish status".to_owned(),
            message: format!("publish = {p}"),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    }

    // R57: Release profile
    let release = table.get("profile").and_then(|p| p.get("release"));

    if let Some(rel) = release {
        if let Some(rel_table) = rel.as_table() {
            let settings: Vec<String> = rel_table
                .iter()
                .map(|(k, v)| format!("{k} = {v}"))
                .collect();
            results.push(CheckResult {
                id: "R57".to_owned(),
                severity: Severity::Info,
                title: "Release profile".to_owned(),
                message: settings.join(", "),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
    }
}
