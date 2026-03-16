use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;
/// Expected serde/toml/yaml deserialization method bans for Garde boundary validation.
pub const EXPECTED_SERDE_METHOD_BANS: &[&str] = &[
    "serde_json::from_str",
    "serde_json::from_slice",
    "serde_json::from_value",
    "serde_json::from_reader",
    "toml::from_str",
    "serde_yaml::from_str",
    "serde_yaml::from_reader",
];

/// Expected axum extractor type bans for Garde boundary validation.
pub const EXPECTED_AXUM_TYPE_BANS: &[&str] = &[
    "axum::extract::Json",
    "axum::Json",
    "axum::extract::Query",
    "axum::extract::Form",
];

/// Orchestrator: run all Garde boundary validation checks.
pub fn check(fs: &dyn FileSystem, workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R-GARDE-01: garde in workspace dependencies
    let cargo_path = workspace_root.join("Cargo.toml");
    let cargo_content = fs.read_file(&cargo_path);
    results.extend(check_garde_dependency(cargo_content.as_deref()));

    // R-GARDE-02 through R-GARDE-04: clippy.toml ban checks
    let clippy_path = workspace_root.join("clippy.toml");
    if let Some(clippy_content) = fs.read_file(&clippy_path) {
        let file_display = clippy_path.display().to_string();
        match clippy_content.parse::<toml::Value>() {
            Ok(table) => {
                results.extend(check_ban_presence(
                    &table,
                    "disallowed-methods",
                    EXPECTED_SERDE_METHOD_BANS,
                    "R-GARDE-02",
                    "Garde serde method ban",
                    &file_display,
                ));
                results.extend(check_ban_presence(
                    &table,
                    "disallowed-types",
                    EXPECTED_AXUM_TYPE_BANS,
                    "R-GARDE-03",
                    "Garde axum type ban",
                    &file_display,
                ));
                results.extend(check_reqwest_json_ban_from_table(&table, &file_display));
            }
            Err(e) => {
                results.push(CheckResult {
                    id: "R-GARDE-02".to_owned(),
                    severity: Severity::Warn,
                    title: "clippy.toml parse error".to_owned(),
                    message: format!("Cannot check Garde bans: {e}"),
                    file: Some(file_display),
                    line: None,
                    inventory: false,
                });
            }
        }
    } else {
        results.push(CheckResult {
            id: "R-GARDE-02".to_owned(),
            severity: Severity::Warn,
            title: "clippy.toml not found".to_owned(),
            message: "Cannot check Garde method/type bans without clippy.toml".to_owned(),
            file: Some(clippy_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // R-GARDE-05: derive inventory scan (excludes test files)
    let rs_files: Vec<String> = super::source_scan::collect_rs_files(workspace_root)
        .into_iter()
        .filter(|f| !super::source_scan::is_test_path(f))
        .collect();
    results.extend(check_derive_inventory(fs, &rs_files, workspace_root));

    results
}

// ---------------------------------------------------------------------------
// R-GARDE-01: garde dependency presence
// ---------------------------------------------------------------------------

pub fn check_garde_dependency(cargo_content: Option<&str>) -> Vec<CheckResult> {
    let Some(content) = cargo_content else {
        return vec![CheckResult {
            id: "R-GARDE-01".to_owned(),
            severity: Severity::Info,
            title: "Cargo.toml not found".to_owned(),
            message: "Cannot check for garde dependency".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory()];
    };

    if content_has_garde_dependency(content) {
        vec![CheckResult {
            id: "R-GARDE-01".to_owned(),
            severity: Severity::Info,
            title: "garde dependency found".to_owned(),
            message: "garde (runtime validation library) is listed in workspace or crate dependencies. Enables `#[derive(Validate)]` for input boundary struct validation. No action needed.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory()]
    } else {
        vec![CheckResult {
            id: "R-GARDE-01".to_owned(),
            severity: Severity::Error,
            title: "garde dependency missing".to_owned(),
            message: "garde is not in [workspace.dependencies] or [dependencies]. Every project MUST have garde for runtime input validation at adapter boundaries. Without it, deserialized input is not validated, risking data corruption. Add `garde = { version = \"...\", features = [\"derive\"] }` to [workspace.dependencies].".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }]
    }
}

/// Check if content contains a garde dependency entry.
/// Looks for lines like `garde = ` or `garde = {` in dependency sections.
pub fn content_has_garde_dependency(content: &str) -> bool {
    let mut in_deps_section = false;
    for line in content.lines() {
        let trimmed = line.trim();

        // Detect dependency sections
        if trimmed.starts_with('[') {
            let lower = trimmed.to_lowercase();
            in_deps_section = lower.contains("dependencies");
            continue;
        }

        if in_deps_section && trimmed.starts_with("garde") {
            // Match "garde =" or "garde=" but not "garde_something"
            let after = trimmed.strip_prefix("garde");
            if let Some(rest) = after {
                let rest_trimmed = rest.trim_start();
                if rest_trimmed.starts_with('=') {
                    return true;
                }
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// R-GARDE-04: reqwest::Response::json ban in clippy.toml
// ---------------------------------------------------------------------------

pub fn check_reqwest_json_ban_from_table(table: &toml::Value, file: &str) -> Vec<CheckResult> {
    check_ban_presence(
        table,
        "disallowed-methods",
        &["reqwest::Response::json"],
        "R-GARDE-04",
        "Garde reqwest method ban",
        file,
    )
}

// ---------------------------------------------------------------------------
// R-GARDE-05: derive inventory scan
// ---------------------------------------------------------------------------

/// The four input boundary derives that require `Validate`.
pub const INPUT_BOUNDARY_DERIVES: &[&str] = &["Deserialize", "Parser", "Args", "FromRow"];

pub fn check_derive_inventory(
    fs: &dyn FileSystem,
    rs_files: &[String],
    workspace_root: &Path,
) -> Vec<CheckResult> {
    let mut with_validate: usize = 0;
    let mut without_validate: usize = 0;

    for file_path in rs_files {
        let path = Path::new(file_path);
        let Some(content) = fs.read_file(path) else {
            continue;
        };
        let Some(parsed) = super::ast_helpers::parse_file(&content) else {
            continue;
        };
        let derives = super::ast_helpers::find_derive_attributes(&parsed);
        let (w, wo) = count_unvalidated_input_structs(&derives);
        with_validate = with_validate.saturating_add(w);
        without_validate = without_validate.saturating_add(wo);
    }

    let severity = if without_validate > 0 {
        Severity::Error
    } else {
        Severity::Info
    };

    let message = if without_validate == 0 {
        format!(
            "{with_validate} input boundary structs (Deserialize/Parser/Args/FromRow) all derive `Validate` for runtime input validation. No structs are missing validation. No action needed."
        )
    } else {
        format!(
            "{with_validate} input boundary structs have `Validate`, but {without_validate} structs deriving Deserialize/Parser/Args/FromRow are missing `#[derive(Validate)]`. Without `Validate`, deserialized input bypasses runtime validation. Add `#[derive(Validate)]` and garde field attributes to each missing struct."
        )
    };
    let result = CheckResult {
        id: "R-GARDE-05".to_owned(),
        severity,
        title: "Input boundary struct validation inventory".to_owned(),
        message,
        file: Some(workspace_root.display().to_string()),
        line: None,
        inventory: false,
    };
    // Mark as inventory when nothing is missing (confirmation only)
    vec![if without_validate == 0 { result.as_inventory() } else { result }]
}

/// Check if a macro name matches any of the input boundary derives,
/// accounting for path-qualified forms like `serde::Deserialize` or `clap::Parser`.
pub fn is_input_boundary_derive(macro_name: &str) -> bool {
    INPUT_BOUNDARY_DERIVES.iter().any(|&d| {
        macro_name == d || macro_name.ends_with(&format!("::{d}"))
    })
}

/// Count structs that derive any input boundary trait (`Deserialize`, `Parser`, `Args`, `FromRow`)
/// and check whether they also derive `Validate`.
pub fn count_unvalidated_input_structs(
    derives: &[super::ast_helpers::DeriveInfo],
) -> (usize, usize) {
    let mut with_validate: usize = 0;
    let mut without_validate: usize = 0;

    for info in derives {
        let has_input_boundary = info
            .macros
            .iter()
            .any(|m| is_input_boundary_derive(m));
        if !has_input_boundary {
            continue;
        }
        let has_validate = info
            .macros
            .iter()
            .any(|m| m == "Validate" || m.ends_with("::Validate"));
        if has_validate {
            with_validate = with_validate.saturating_add(1);
        } else {
            without_validate = without_validate.saturating_add(1);
        }
    }

    (with_validate, without_validate)
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract paths from a clippy.toml ban array (disallowed-methods or disallowed-types).
pub fn extract_ban_paths(table: &toml::Value, key: &str) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    if let Some(bans) = table.get(key).and_then(|v| v.as_array()) {
        for ban in bans {
            if let Some(path) = ban.get("path").and_then(|p| p.as_str()) {
                let _ = paths.insert(path.to_owned());
            } else if let Some(path) = ban.as_str() {
                let _ = paths.insert(path.to_owned());
            }
        }
    }
    paths
}

/// Check that all expected bans are present in a clippy.toml ban list.
/// Returns a single Warn if any are missing (listing the missing ones),
/// or a single Info if all are present.
pub fn check_ban_presence(
    table: &toml::Value,
    key: &str,
    expected: &[&str],
    check_id: &str,
    label: &str,
    file: &str,
) -> Vec<CheckResult> {
    let found = extract_ban_paths(table, key);
    let missing: Vec<&str> = expected
        .iter()
        .filter(|e| !found.contains(**e))
        .copied()
        .collect();

    if missing.is_empty() {
        vec![CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Info,
            title: format!("All {label}s present"),
            message: format!("All {} expected {label}s found in clippy.toml {key}. These bans force usage of validated wrapper types instead of raw deserialization. No action needed.", expected.len()),
            file: Some(file.to_owned()),
            line: None,
            inventory: false,
        }.as_inventory()]
    } else {
        vec![CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Warn,
            title: format!("Missing {label}s"),
            message: format!("Missing from clippy.toml {key}: {}. Without these bans, code can bypass the Validated<T> wrapper and deserialize input without runtime validation. Add them to clippy.toml or run `guardrail3 generate`.", missing.join(", ")),
            file: Some(file.to_owned()),
            line: None,
            inventory: false,
        }]
    }
}

