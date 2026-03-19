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
        results.push(CheckResult {
            id: "R55".to_owned(),
            severity: Severity::Error,
            title: "Cargo.toml unreadable".to_owned(),
            message: "Failed to read workspace Cargo.toml for metadata checks".to_owned(),
            file: Some(cargo_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R55".to_owned(),
                severity: Severity::Error,
                title: "Cargo.toml parse error".to_owned(),
                message: format!("Invalid TOML in workspace Cargo.toml: {e}"),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
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
        results.push(
            CheckResult {
                id: "R55".to_owned(),
                severity: Severity::Info,
                title: "Workspace metadata".to_owned(),
                message: format!("Workspace Cargo.toml metadata: {}.", meta_parts.join(", ")),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }

    // R55: edition must be present and modern (2021 or 2024)
    match edition {
        Some(ed) if ed == "2021" || ed == "2024" => {}
        Some(ed) => {
            results.push(CheckResult {
                id: "R55".to_owned(),
                severity: Severity::Warn,
                title: "Outdated Rust edition".to_owned(),
                message: format!(
                    "Workspace edition is `{ed}`. Use edition `2024` (or `2021` minimum)."
                ),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R55".to_owned(),
                severity: Severity::Warn,
                title: "Rust edition not set in workspace".to_owned(),
                message: "No `edition` in workspace package metadata. Defaults to 2015. Set `edition = \"2024\"` in `[workspace.package]`.".to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_publish_status(table: &toml::Value, cargo_path: &Path, results: &mut Vec<CheckResult>) {
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
            message: format!("publish = {p}. Controls whether `cargo publish` is allowed for this crate. Informational, no action needed."),
            file: Some(cargo_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    }
}

fn check_release_profile(table: &toml::Value, cargo_path: &Path, results: &mut Vec<CheckResult>) {
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
                message: format!("[profile.release] settings: {}. These optimize the release binary (e.g., LTO, strip, codegen-units). Informational, no action needed.", settings.join(", ")),
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
