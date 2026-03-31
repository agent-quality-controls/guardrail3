//! R-TOPOLOGY-01: Hexarch structural enforcement.
//!
//! Auto-detects service apps from `apps/*/Cargo.toml` and enforces the
//! canonical hexarch directory template:
//!
//! ```text
//! apps/{name}/crates/{adapters/{inbound,outbound}, app, domain, ports/{inbound,outbound}}
//! ```
//!
//! Rules:
//! - `src/` at app root is banned — code must be in `crates/`
//! - `crates/` must contain exactly `{adapters, app, domain, ports}`
//! - `adapters/` and `ports/` must each contain exactly `{inbound, outbound}`
//! - Container folders (app, domain, ports/inbound, ports/outbound,
//!   adapters/inbound, adapters/outbound) must have `.gitkeep` or at least one
//!   crate subdir with `Cargo.toml`
//! - Only `.gitkeep` files are allowed in structural and container dirs

use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// Run all R-TOPOLOGY-01 structural checks.
///
/// Auto-detects service apps by scanning `apps/*/Cargo.toml`.
pub fn check_hexarch_structure(
    fs: &dyn FileSystem,
    root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let apps_dir = root.join("apps");
    let apps_entries = fs.list_dir(&apps_dir);
    if apps_entries.is_empty() {
        return;
    }

    for entry in &apps_entries {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let app_dir = root.join("apps").join(&name);

        // Only check apps that have a Cargo.toml (i.e. Rust service apps)
        if fs.read_file(&app_dir.join("Cargo.toml")).is_none() {
            continue;
        }

        check_single_app(fs, root, &name, &app_dir, results);
    }
}

fn check_single_app(
    fs: &dyn FileSystem,
    _root: &Path,
    name: &str,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Ban src/ at app root
    let src_entries = fs.list_dir(&app_dir.join("src"));
    if !src_entries.is_empty() {
        results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!("Service `{name}` has src/ directory"),
    format!(
                "Service `{name}` has an `src/` directory. Code must be in `crates/` \
                 following hexarch layout. Move code into \
                 `crates/{{adapters,app,domain,ports}}` subcrates."
            ),
    Some(app_dir.join("src").display().to_string()),
    None,
    false,
        ));
    }

    check_crates_dir(fs, name, app_dir, "crates", results);,
)

/// Check a `crates/` directory for hexarch structure.
/// Reusable for both top-level apps and hex-in-hex recursion.
fn check_crates_dir(
    fs: &dyn FileSystem,
    name: &str,
    parent_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    let crates_dir = parent_dir.join("crates");
    let crates_entries = fs.list_dir(&crates_dir);
    if crates_entries.is_empty() {
        results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!("Service `{name}` missing {label_prefix}/ directory"),
    format!(
                "Service `{name}` has no `{label_prefix}/` directory. Create it with the hexarch \
                 template: `{label_prefix}/{{adapters/{{inbound,outbound}}, app, domain, \
                 ports/{{inbound,outbound}}}}`."
            ),
    Some(parent_dir.display().to_string()),
    None,
    false,
        ));
        return;
    }

    let crate_dir_names = list_dir_names(fs, &crates_dir);
    let expected_top = ["adapters", "app", "domain", "ports"];
    for expected in &expected_top {
        if !crate_dir_names.iter().any(|n| n == expected) {
            results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!("Service `{name}` missing {label_prefix}/{expected}/ directory"),
    format!(
                    "Service `{name}` is missing `{label_prefix}/{expected}/`. Create it and add a \
                     `.gitkeep` if not needed yet."
                ),
    Some(crates_dir.display().to_string()),
    None,
    false,
            });
        }
    }

    check_loose_files(fs, name, &crates_dir, label_prefix, results);

    for dir_name in &crate_dir_names {
        if !expected_top.contains(&dir_name.as_str()) {
            results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!(
                    "Service `{name}` has unexpected directory {label_prefix}/{dir_name}/"
                ),
    format!(
                    "Service `{name}` has `{label_prefix}/{dir_name}/` which is not part of the hex \
                     topology template. Only `{{adapters, app, domain, ports}}` directories are \
                     allowed in `{label_prefix}/`."
                ),
    Some(crates_dir.join(dir_name).display().to_string()),
    None,
    false,
            });
        }
    }

    let adapters_label = format!("{label_prefix}/adapters");
    let ports_label = format!("{label_prefix}/ports");
    check_inbound_outbound(fs, name, &crates_dir.join("adapters"), &adapters_label, results);
    check_inbound_outbound(fs, name, &crates_dir.join("ports"), &ports_label, results);

    let app_label = format!("{label_prefix}/app");
    let domain_label = format!("{label_prefix}/domain");
    validate_container_folder(fs, name, &crates_dir.join("app"), &app_label, results);
    validate_container_folder(fs, name, &crates_dir.join("domain"), &domain_label, results);

    for parent in &["adapters", "ports"] {
        for child in &["inbound", "outbound"] {
            let path = crates_dir.join(parent).join(child);
            let label = format!("{label_prefix}/{parent}/{child}");
            validate_container_folder(fs, name, &path, &label, results);
        }
    },
)

/// Check that a structural dir contains exactly {inbound, outbound}.
fn check_inbound_outbound(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    layer: &str,
    results: &mut Vec<CheckResult>,
) {
    let entries = fs.list_dir(dir);
    if entries.is_empty() {
        // Directory doesn't exist — already reported as missing from crates/
        return;
    }

    let dir_names = list_dir_names(fs, dir);
    for expected in &["inbound", "outbound"] {
        if !dir_names.iter().any(|n| n == expected) {
            results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!(
                    "Service `{name}` missing {layer}/{expected}/ directory"
                ),
    format!(
                    "Service `{name}` is missing `{layer}/{expected}/`. \
                     Create it and add a `.gitkeep` if not needed yet."
                ),
    Some(dir.display().to_string()),
    None,
    false,
            ));
        }
    }

    // Check for unexpected dirs in adapters/ or ports/
    for dir_name in &dir_names {
        if dir_name != "inbound" && dir_name != "outbound" {
            results.push(CheckResult {
                id: "R-TOPOLOGY-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` has unexpected directory {layer}/{dir_name}/"
                ),
                message: format!(
                    "Service `{name}` has `{layer}/{dir_name}/` which is not part of \
                     the hexarch template. Only `{{inbound, outbound}}` directories are \
                     allowed in `{layer}/`."
                ),
                file: Some(dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // Check for loose files (not .gitkeep) in structural dirs
    check_loose_files(fs, name, dir, layer, results);,
)

/// Validate a container folder: must have `.gitkeep` or at least one crate subdir.
fn validate_container_folder(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    // Check if directory exists at all (list_dir returns empty for both
    // "doesn't exist" and "exists but empty" — we need to distinguish)
    if fs.metadata(dir).is_none() {
        // Directory doesn't exist — already reported elsewhere
        return;
    }

    let entries = fs.list_dir(dir);
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let has_gitkeep_file = has_gitkeep(fs, dir);

    for entry in &entries {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_dir() {
            dirs.push(entry_name);
        } else {
            files.push(entry_name);
        }
    }

    // Container must have .gitkeep or at least one crate subdir
    if dirs.is_empty() && !has_gitkeep_file {
        let detail = if files.is_empty() {
            "is empty".to_owned()
        } else {
            format!(
                "contains files ({}) but no crate subdirectories",
                files.join(", ")
            )
        };
        results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!("Service `{name}` empty container {label}/"),
    format!(
                "Service `{name}` container `{label}/` {detail}. \
                 Each subdirectory must be a crate with its own `Cargo.toml`, \
                 or add a `.gitkeep` if this layer is not needed yet."
            ),
    Some(dir.display().to_string()),
    None,
    false,
        ));
        // Don't return — still check for loose files below
    }

    // Each subdir in a container must be either:
    // - a crate (has Cargo.toml with [package]), or
    // - a hex-in-hex (has crates/ dir) — recurse structural checks
    for subdir in &dirs {
        let sub_path = dir.join(subdir);
        let has_cargo = fs.read_file(&sub_path.join("Cargo.toml")).is_some();
        let has_crates = !fs.list_dir(&sub_path.join("crates")).is_empty();

        if has_crates {
            // Hex-in-hex: recurse structural checks
            let inner_label = format!("{label}/{subdir}/crates");
            check_crates_dir(fs, name, &sub_path, &inner_label, results);
        } else if !has_cargo {
            results.push(CheckResult {
                id: "R-TOPOLOGY-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` subdirectory {label}/{subdir}/ missing Cargo.toml"
                ),
                message: format!(
                    "Service `{name}` has `{label}/{subdir}/` but it has no `Cargo.toml` \
                     and no `crates/` directory. Every subdirectory in a container folder \
                     must be its own crate or a hex-in-hex with its own `crates/` structure."
                ),
                file: Some(sub_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // Check for loose files (only .gitkeep is allowed)
    check_loose_files(fs, name, dir, label, results);,
)

/// Report loose files in a directory (only `.gitkeep` is allowed).
fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    let entries = fs.list_dir(dir);
    let mut bad_files: Vec<String> = Vec::new();

    for entry in &entries {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() && entry_name != ".gitkeep" {
            bad_files.push(entry_name);
        }
    }

    if !bad_files.is_empty() {
        results.push(CheckResult::from_parts(
    "R-TOPOLOGY-01".to_owned(),
    Severity::Error,
    format!(
                "Service `{name}` has loose files in {label}/"
            ),
    format!(
                "Service `{name}` has files in `{label}/` that don't belong: {}. \
                 Only `.gitkeep` is allowed in structural/container directories. \
                 Move code into crate subdirectories.",
                bad_files.join(", ")
            ),
    Some(dir.display().to_string()),
    None,
    false,
        ));
    }

/// List subdirectory names in a directory.
fn list_dir_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    let entries = fs.list_dir(dir);
    let mut names = Vec::new();
    for entry in &entries {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names,
)

/// Check if a directory contains a `.gitkeep` file.
fn has_gitkeep(fs: &dyn FileSystem, dir: &Path) -> bool {
    fs.read_file(&dir.join(".gitkeep")).is_some(),
)
