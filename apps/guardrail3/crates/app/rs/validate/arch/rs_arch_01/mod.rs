//! RS-ARCH-01: Hex arch structural enforcement.
//!
//! Auto-detects service apps from `apps/*/Cargo.toml` and enforces the
//! canonical hex arch directory template.

mod check_01_crates_exists;
mod check_02_exact_contents;
mod check_03_inbound_outbound;
mod check_05_container_not_empty;
mod check_06_leaf_valid;
mod check_12_src_banned;
pub mod helpers;

use std::path::Path;

use crate::domain::report::CheckResult;
use crate::ports::outbound::FileSystem;

/// Run all RS-ARCH-01 structural checks.
///
/// Auto-detects service apps by scanning `apps/*/Cargo.toml`.
pub fn check_hex_arch_structure(fs: &dyn FileSystem, root: &Path, results: &mut Vec<CheckResult>) {
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

        if fs.read_file(&app_dir.join("Cargo.toml")).is_none() {
            continue;
        }

        check_single_app(fs, &name, &app_dir, results);
    }
}

fn check_single_app(
    fs: &dyn FileSystem,
    name: &str,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    check_12_src_banned::check(fs, name, app_dir, results);
    check_crates_dir(fs, name, app_dir, "crates", results);
}

/// Check a `crates/` directory for hex arch structure.
/// Reusable for both top-level apps and hex-in-hex recursion.
fn check_crates_dir(
    fs: &dyn FileSystem,
    name: &str,
    parent_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    if !check_01_crates_exists::check(fs, name, parent_dir, label_prefix, results) {
        return;
    }

    let crates_dir = parent_dir.join("crates");
    check_02_exact_contents::check(fs, name, &crates_dir, label_prefix, results);

    let adapters_label = format!("{label_prefix}/adapters");
    let ports_label = format!("{label_prefix}/ports");
    check_03_inbound_outbound::check(
        fs,
        name,
        &crates_dir.join("adapters"),
        &adapters_label,
        results,
    );
    check_03_inbound_outbound::check(fs, name, &crates_dir.join("ports"), &ports_label, results);

    // Container validation: app, domain
    let app_label = format!("{label_prefix}/app");
    let domain_label = format!("{label_prefix}/domain");
    check_05_container_not_empty::check(fs, name, &crates_dir.join("app"), &app_label, results);
    check_05_container_not_empty::check(
        fs,
        name,
        &crates_dir.join("domain"),
        &domain_label,
        results,
    );
    check_06_leaf_valid::check(
        fs,
        name,
        &crates_dir.join("app"),
        &app_label,
        results,
        &check_crates_dir,
    );
    check_06_leaf_valid::check(
        fs,
        name,
        &crates_dir.join("domain"),
        &domain_label,
        results,
        &check_crates_dir,
    );

    // Container validation: adapters/{in,out}, ports/{in,out}
    for parent in &["adapters", "ports"] {
        for child in &["inbound", "outbound"] {
            let path = crates_dir.join(parent).join(child);
            let label = format!("{label_prefix}/{parent}/{child}");
            check_05_container_not_empty::check(fs, name, &path, &label, results);
            check_06_leaf_valid::check(fs, name, &path, &label, results, &check_crates_dir);
        }
    }
}
