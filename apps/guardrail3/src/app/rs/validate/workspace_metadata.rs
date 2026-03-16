use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

// R55-R57: workspace metadata & release profile
pub fn check_workspace_metadata(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let cargo_path = workspace_root.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let Some(content) = fs.read_file(&cargo_path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    check_edition_and_rust_version(&table, &cargo_path, results);
    check_publish_status(&table, &cargo_path, results);
    check_release_profile(&table, &cargo_path, results);
}

fn check_edition_and_rust_version(
    table: &toml::Value,
    cargo_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let edition = get_package_str_field(table, "edition");
    let rust_version = get_package_str_field(table, "rust-version");

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
            inventory: false,
        }.as_inventory());
    }
}

fn check_publish_status(
    table: &toml::Value,
    cargo_path: &Path,
    results: &mut Vec<CheckResult>,
) {
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
            inventory: false,
        }.as_inventory());
    }
}

fn check_release_profile(
    table: &toml::Value,
    cargo_path: &Path,
    results: &mut Vec<CheckResult>,
) {
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
                inventory: false,
            }.as_inventory());
        }
    }
}

/// Look up a string field in `[workspace.package]`, falling back to `[package]`.
fn get_package_str_field<'a>(table: &'a toml::Value, field: &str) -> Option<&'a str> {
    table
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get(field))
        .and_then(|v| v.as_str())
        .or_else(|| {
            table
                .get("package")
                .and_then(|p| p.get(field))
                .and_then(|v| v.as_str())
        })
}
