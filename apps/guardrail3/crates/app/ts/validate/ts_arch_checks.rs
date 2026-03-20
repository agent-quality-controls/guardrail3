use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_ts_dir;
use crate::domain::report::{CheckResult, Severity, TsAppContext};
use crate::ports::outbound::FileSystem;

// -----------------------------------------------------------------------
// T-ARCH-01: TS service missing hex arch structure
// -----------------------------------------------------------------------

/// Scan `apps/` for TypeScript apps and check that each has
/// `src/modules/domain/` and `src/modules/adapters/` subdirectories.
pub fn check_hex_arch_structure(fs: &dyn FileSystem, root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let apps = discover_ts_apps(fs, root);
    for app_dir in &apps {
        check_single_app_structure(fs, app_dir, &mut results);
    }
    results
}

/// Discover TypeScript apps under `<root>/apps/`.
/// An app is a subdirectory that has TypeScript files (.ts, .tsx) or `package.json`.
/// Rust-only apps (no TS files, no package.json) are skipped.
pub fn discover_ts_apps(fs: &dyn FileSystem, root: &Path) -> Vec<std::path::PathBuf> {
    let apps_dir = root.join("apps");
    let mut found = Vec::new();

    // Use list_dir to enumerate entries under apps/
    for entry in fs.list_dir(&apps_dir) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        // Must have package.json to be considered a TS app
        let has_package_json = fs.read_file(&path.join("package.json")).is_some();
        if !has_package_json {
            // No package.json — check if there are any .ts/.tsx files
            if !has_ts_files(&path) {
                continue; // Rust-only app, skip
            }
        }
        found.push(path);
    }
    found
}

/// Run hex arch structure checks only on service-type apps.
pub fn check_hex_arch_structure_for_apps(
    fs: &dyn FileSystem,
    app_contexts: &[TsAppContext],
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for ctx in app_contexts {
        if ctx.categories.architecture {
            check_single_app_structure(fs, &ctx.path, &mut results);
        }
    }
    results
}

/// Run import boundary checks only on service-type apps.
pub fn check_import_boundaries_for_apps(
    fs: &dyn FileSystem,
    app_contexts: &[TsAppContext],
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for ctx in app_contexts {
        if ctx.categories.architecture {
            let ts_files = collect_module_ts_files(&ctx.path);
            for file_path_str in &ts_files {
                let file_path = Path::new(file_path_str);
                let Some(content) = fs.read_file(file_path) else {
                    continue;
                };
                check_file_imports(file_path, &content, &mut results);
            }
        }
    }
    results
}

/// Check if a directory contains any TypeScript files (.ts, .tsx).
fn has_ts_files(dir: &Path) -> bool {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
        .any(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|e| e == "ts" || e == "tsx")
        })
}

/// Check a single TS app for hex arch structure.
pub fn check_single_app_structure(
    fs: &dyn FileSystem,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    let app_name = app_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let modules_dir = app_dir.join("src").join("modules");

    // src/modules/ must exist
    if fs.metadata(&modules_dir).is_none() {
        results.push(CheckResult {
            id: "T-ARCH-01".to_owned(),
            severity: Severity::Warn,
            title: format!("TS app `{app_name}` missing src/modules/ directory"),
            message: format!(
                "App `{app_name}` has no `src/modules/` directory. Create it with the hex arch \
                 template: `src/modules/{{domain, ports/{{inbound,outbound}}, application, \
                 adapters/{{inbound,outbound}}}}`."
            ),
            file: Some(app_dir.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    check_ts_modules_dir(fs, app_name, &modules_dir, "src/modules", results);
}

/// Check a `modules/` directory for TS hex arch structure.
/// Reusable for both top-level apps and hex-in-hex recursion.
fn check_ts_modules_dir(
    fs: &dyn FileSystem,
    name: &str,
    modules_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    // modules/ must contain exactly {adapters, application, domain, ports}
    let expected_top = ["adapters", "application", "domain", "ports"];
    let dir_names = list_ts_dir_names(modules_dir);

    for expected in &expected_top {
        if !dir_names.iter().any(|n| n == expected) {
            results.push(CheckResult {
                id: "T-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "TS app `{name}` missing {label_prefix}/{expected}/ directory"
                ),
                message: format!(
                    "App `{name}` is missing `{label_prefix}/{expected}/`. Create it and add a \
                     `.gitkeep` if not needed yet."
                ),
                file: Some(modules_dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // No unexpected dirs
    for dir_name in &dir_names {
        if !expected_top.contains(&dir_name.as_str()) {
            results.push(CheckResult {
                id: "T-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "TS app `{name}` has unexpected directory {label_prefix}/{dir_name}/"
                ),
                message: format!(
                    "App `{name}` has `{label_prefix}/{dir_name}/` which is not part of the hex \
                     arch template. Only `{{adapters, application, domain, ports}}` directories \
                     are allowed in `{label_prefix}/`."
                ),
                file: Some(modules_dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // No loose files in modules/ (structural dir)
    check_ts_loose_files(fs, name, modules_dir, label_prefix, results);

    // adapters/ and ports/ must each contain {inbound, outbound}
    let adapters_label = format!("{label_prefix}/adapters");
    let ports_label = format!("{label_prefix}/ports");
    check_ts_inbound_outbound(
        fs,
        name,
        &modules_dir.join("adapters"),
        &adapters_label,
        results,
    );
    check_ts_inbound_outbound(
        fs,
        name,
        &modules_dir.join("ports"),
        &ports_label,
        results,
    );

    // Validate container folders: domain, application, adapters/{in,out}, ports/{in,out}
    let domain_label = format!("{label_prefix}/domain");
    let application_label = format!("{label_prefix}/application");
    validate_ts_container(fs, name, &modules_dir.join("domain"), &domain_label, results);
    validate_ts_container(
        fs,
        name,
        &modules_dir.join("application"),
        &application_label,
        results,
    );
    for parent in &["adapters", "ports"] {
        for child in &["inbound", "outbound"] {
            let path = modules_dir.join(parent).join(child);
            let label = format!("{label_prefix}/{parent}/{child}");
            validate_ts_container(fs, name, &path, &label, results);
        }
    }
}

/// Check that a structural dir contains exactly {inbound, outbound}.
fn check_ts_inbound_outbound(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    if fs.metadata(dir).is_none() {
        return; // missing dir already reported
    }

    let dir_names = list_ts_dir_names(dir);
    for expected in &["inbound", "outbound"] {
        if !dir_names.iter().any(|n| n == expected) {
            results.push(CheckResult {
                id: "T-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!("TS app `{name}` missing {label}/{expected}/ directory"),
                message: format!(
                    "App `{name}` is missing `{label}/{expected}/`. \
                     Create it and add a `.gitkeep` if not needed yet."
                ),
                file: Some(dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    for dir_name in &dir_names {
        if dir_name != "inbound" && dir_name != "outbound" {
            results.push(CheckResult {
                id: "T-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "TS app `{name}` has unexpected directory {label}/{dir_name}/"
                ),
                message: format!(
                    "App `{name}` has `{label}/{dir_name}/` which is not part of the hex \
                     arch template. Only `{{inbound, outbound}}` directories are \
                     allowed in `{label}/`."
                ),
                file: Some(dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    check_ts_loose_files(fs, name, dir, label, results);
}

/// Validate a TS container folder: must have `.gitkeep` or at least one subdir.
fn validate_ts_container(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    if fs.metadata(dir).is_none() {
        return; // missing dir already reported
    }

    let dirs = list_ts_dir_names(dir);
    let has_gitkeep = fs.read_file(&dir.join(".gitkeep")).is_some();

    if dirs.is_empty() && !has_gitkeep {
        results.push(CheckResult {
            id: "T-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("TS app `{name}` empty container {label}/"),
            message: format!(
                "App `{name}` container `{label}/` has no subdirectories. \
                 Add module subdirectories or a `.gitkeep` if this layer is not needed yet."
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // Each subdir must have at least one .ts/.tsx file or be a hex-in-hex (has modules/ inside)
    for subdir in &dirs {
        let sub_path = dir.join(subdir);
        let has_modules = fs.metadata(&sub_path.join("modules")).is_some();

        if has_modules {
            // Hex-in-hex: recurse
            let inner_label = format!("{label}/{subdir}/modules");
            check_ts_modules_dir(fs, name, &sub_path.join("modules"), &inner_label, results);
        } else if !has_ts_source_files(&sub_path)
            && fs.read_file(&sub_path.join(".gitkeep")).is_none()
        {
            results.push(CheckResult {
                id: "T-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "TS app `{name}` subdirectory {label}/{subdir}/ has no .ts/.tsx files"
                ),
                message: format!(
                    "App `{name}` has `{label}/{subdir}/` but it contains no TypeScript files. \
                     Every subdirectory in a container folder must be a module with .ts/.tsx files, \
                     a hex-in-hex with its own `modules/` structure, or have a `.gitkeep` placeholder."
                ),
                file: Some(sub_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    check_ts_loose_files(fs, name, dir, label, results);
}

/// Report loose files in a directory (only `.gitkeep` is allowed).
fn check_ts_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    let mut bad_files: Vec<String> = Vec::new();
    for entry in fs.list_dir(dir) {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() && entry_name != ".gitkeep" {
            bad_files.push(entry_name);
        }
    }

    if !bad_files.is_empty() {
        results.push(CheckResult {
            id: "T-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("TS app `{name}` has loose files in {label}/"),
            message: format!(
                "App `{name}` has files in `{label}/` that don't belong: {}. \
                 Only `.gitkeep` is allowed in structural/container directories. \
                 Move code into module subdirectories.",
                bad_files.join(", ")
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// List subdirectory names in a directory.
fn list_ts_dir_names(dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                names.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    names
}

/// Check if a directory contains any .ts or .tsx files (recursively).
fn has_ts_source_files(dir: &Path) -> bool {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
        .any(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|e| e == "ts" || e == "tsx")
        })
}

// -----------------------------------------------------------------------
// T-ARCH-02: TS import boundary violation
// -----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsLayer {
    Domain,
    Ports,
    Application,
    Adapters,
}

impl TsLayer {
    /// Returns the layers that this layer is NOT allowed to import from.
    pub const fn forbidden(self) -> &'static [Self] {
        match self {
            Self::Domain => &[Self::Application, Self::Adapters, Self::Ports],
            Self::Ports => &[Self::Application, Self::Adapters],
            Self::Application => &[Self::Adapters],
            Self::Adapters => &[],
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Ports => "ports",
            Self::Application => "application",
            Self::Adapters => "adapters",
        }
    }
}

/// Determine which layer a file belongs to based on its path.
pub fn layer_from_path(path: &Path) -> Option<TsLayer> {
    let path_str = path.display().to_string();
    // Look for /modules/<layer>/ in the path
    let segments: Vec<&str> = path_str.split('/').collect();
    let mut found_modules = false;
    for seg in &segments {
        if found_modules {
            return match *seg {
                "domain" => Some(TsLayer::Domain),
                "ports" => Some(TsLayer::Ports),
                "application" => Some(TsLayer::Application),
                "adapters" => Some(TsLayer::Adapters),
                _ => None,
            };
        }
        if *seg == "modules" {
            found_modules = true;
        }
    }
    None
}

/// Determine which layer an import target refers to.
fn layer_from_import(import_path: &str, file_dir: &Path) -> Option<TsLayer> {
    // Handle alias imports: @/modules/..., ~/modules/...
    if let Some(rest) = import_path
        .strip_prefix("@/modules/")
        .or_else(|| import_path.strip_prefix("~/modules/"))
    {
        let first_segment = rest.split('/').next().unwrap_or("");
        return match first_segment {
            "domain" => Some(TsLayer::Domain),
            "ports" => Some(TsLayer::Ports),
            "application" => Some(TsLayer::Application),
            "adapters" => Some(TsLayer::Adapters),
            _ => None,
        };
    }

    // Handle direct layer aliases: @domain/..., @adapters/..., @application/..., @ports/...
    // These are common tsconfig path aliases that map directly to modules/ subdirs
    if import_path.starts_with("@domain") {
        return Some(TsLayer::Domain);
    }
    if import_path.starts_with("@ports") {
        return Some(TsLayer::Ports);
    }
    if import_path.starts_with("@application") {
        return Some(TsLayer::Application);
    }
    if import_path.starts_with("@adapters") {
        return Some(TsLayer::Adapters);
    }

    // Handle relative imports: resolve ../.. segments
    if import_path.starts_with('.') {
        let resolved = resolve_relative(file_dir, import_path);
        return layer_from_resolved_path(&resolved);
    }

    None
}

/// Resolve a relative import path against the file's directory.
pub fn resolve_relative(base: &Path, rel: &str) -> String {
    let base_str = base.to_string_lossy();
    let mut parts: Vec<String> = base_str
        .split('/')
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    for seg in rel.split('/') {
        match seg {
            ".." => {
                let _ = parts.pop();
            }
            "." | "" => {}
            s => parts.push(s.to_owned()),
        }
    }
    parts.join("/")
}

/// Check if a resolved path contains a modules/<layer> segment.
fn layer_from_resolved_path(resolved: &str) -> Option<TsLayer> {
    let segments: Vec<&str> = resolved.split('/').collect();
    let mut found_modules = false;
    for seg in &segments {
        if found_modules {
            return match *seg {
                "domain" => Some(TsLayer::Domain),
                "ports" => Some(TsLayer::Ports),
                "application" => Some(TsLayer::Application),
                "adapters" => Some(TsLayer::Adapters),
                _ => None,
            };
        }
        if *seg == "modules" {
            found_modules = true;
        }
    }
    None
}

/// Extract import path from a line containing `from '...'`, `from "..."`, or `require('...')`.
#[allow(clippy::string_slice)] // reason: all indices are validated ASCII positions from find()
pub fn extract_import_path(line: &str) -> Option<&str> {
    extract_between_after(line, "from '", '\'')
        .or_else(|| extract_between_after(line, "from \"", '"'))
        .or_else(|| extract_between_after(line, "require('", '\''))
        .or_else(|| extract_between_after(line, "require(\"", '"'))
}

/// Find `prefix` in `line`, then extract text between end-of-prefix and next `closing`.
/// All characters in prefixes are ASCII, so byte offsets are safe for slicing.
#[allow(clippy::arithmetic_side_effects)] // reason: prefix.len() added to ASCII-safe find() index
#[allow(clippy::string_slice)] // reason: indices from find() on ASCII delimiters are char-boundary safe
fn extract_between_after<'a>(line: &'a str, prefix: &str, closing: char) -> Option<&'a str> {
    let idx = line.find(prefix)?;
    let start = idx.checked_add(prefix.len())?;
    let rest = line.get(start..)?;
    let end = rest.find(closing)?;
    rest.get(..end)
}

/// Scan all `.ts`/`.tsx` files under `src/modules/` for import boundary violations.
pub fn check_import_boundaries(fs: &dyn FileSystem, root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let ts_files = collect_module_ts_files(root);
    for file_path_str in &ts_files {
        let file_path = Path::new(file_path_str);
        let Some(content) = fs.read_file(file_path) else {
            continue;
        };
        check_file_imports(file_path, &content, &mut results);
    }
    results
}

/// Collect `.ts`/`.tsx` files inside any `src/modules/` directory tree.
fn collect_module_ts_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path_str = entry.path().display().to_string();
        if !is_ts_source(&path_str) {
            continue;
        }
        // Only files inside a modules/ directory, excluding test files
        if path_str.contains("/modules/") && !is_ts_test_file(&path_str) {
            files.push(path_str);
        }
    }
    files
}

/// Check if a TS file is a test file (by path convention).
fn is_ts_test_file(path: &str) -> bool {
    path.contains("/__tests__/")
        || path.contains(".test.")
        || path.contains(".spec.")
        || path.contains("/test/")
        || path.contains("/tests/")
}

fn is_ts_source(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|e| e == "ts" || e == "tsx")
}

/// Check a single file's imports for boundary violations.
pub fn check_file_imports(file_path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(source_layer) = layer_from_path(file_path) else {
        return;
    };
    let forbidden = source_layer.forbidden();
    if forbidden.is_empty() {
        return; // adapters can import anything
    }

    let file_dir = file_path.parent().unwrap_or(file_path);

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // Skip comments
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }
        let Some(import_path) = extract_import_path(trimmed) else {
            continue;
        };
        let Some(target_layer) = layer_from_import(import_path, file_dir) else {
            continue;
        };
        if forbidden.contains(&target_layer) {
            let line_number = line_idx.saturating_add(1);
            results.push(CheckResult {
                id: "T-ARCH-02".to_owned(),
                severity: Severity::Error,
                title: "Hexagonal architecture import boundary violation".to_owned(),
                message: format!(
                    "The `{}` layer imports from the `{}` layer: `{import_path}`. In hexagonal architecture, \
                     imports must flow inward (adapters -> application -> ports -> domain). The `{}` layer \
                     must not depend on `{}`. Move shared types to a common layer, or invert the dependency \
                     using an interface/port.",
                    source_layer.label(),
                    target_layer.label(),
                    source_layer.label(),
                    target_layer.label(),
                ),
                file: Some(file_path.display().to_string()),
                line: Some(line_number),
                inventory: false,
            });
        }
    }
}
