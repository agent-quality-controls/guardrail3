use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_ts_dir;
use guardrail3_app_topology_helpers as topology_helpers;
use guardrail3_domain_report::{CheckResult, Severity, TsAppContext};
use guardrail3_outbound_traits::FileSystem;

const ID: &str = "T-TOPOLOGY-01";
const ENTITY: &str = "TS app";

// -----------------------------------------------------------------------
// T-TOPOLOGY-01: TS service missing hex topology structure
// -----------------------------------------------------------------------

/// Scan `apps/` for TypeScript apps and check that each has
/// `src/modules/domain/` and `src/modules/adapters/` subdirectories.
pub fn check_hex_topology_structure(fs: &dyn FileSystem, root: &Path) -> Vec<CheckResult> {
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

/// Run hex topology structure checks only on service-type apps.
pub fn check_hex_topology_structure_for_apps(
    fs: &dyn FileSystem,
    app_contexts: &[TsAppContext],
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for ctx in app_contexts {
        if ctx.categories().topology() {
            check_single_app_structure(fs, ctx.path(), &mut results);
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
        if ctx.categories().topology() {
            let ts_files = collect_module_ts_files(ctx.path());
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

/// Check a single TS app for hex topology structure.
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
        results.push(CheckResult::from_parts(
            "T-TOPOLOGY-01".to_owned(),
            Severity::Warn,
            format!("TS app `{app_name}` missing src/modules/ directory"),
            format!(
                "App `{app_name}` has no `src/modules/` directory. Create it with the hex topology \
                 template: `src/modules/{{domain, ports/{{inbound,outbound}}, application, \
                 adapters/{{inbound,outbound}}}}`."
            ),
            Some(app_dir.display().to_string()),
            None,
            false,
        ));
        return;
    }

    check_ts_modules_dir(fs, app_name, &modules_dir, "src/modules", results);
}

/// Check a `modules/` directory for TS hex topology structure.
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
    topology_helpers::check_exact_subdirs(
        fs,
        name,
        modules_dir,
        label_prefix,
        &expected_top,
        ID,
        ENTITY,
        results,
    );

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
    check_ts_inbound_outbound(fs, name, &modules_dir.join("ports"), &ports_label, results);

    // Validate container folders: domain, application, adapters/{in,out}, ports/{in,out}
    let domain_label = format!("{label_prefix}/domain");
    let application_label = format!("{label_prefix}/application");
    validate_ts_container(
        fs,
        name,
        &modules_dir.join("domain"),
        &domain_label,
        results,
    );
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

    topology_helpers::check_exact_subdirs(
        fs,
        name,
        dir,
        label,
        &["inbound", "outbound"],
        ID,
        ENTITY,
        results,
    );
}

/// Validate a TS container folder: must have `.gitkeep` or at least one subdir.
/// Also checks for loose files (via topology_helpers::check_container_not_empty)
/// and validates each subdir has .ts/.tsx files or is a hex-in-hex.
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

    // Empty check + loose file detection (shared helper)
    topology_helpers::check_container_not_empty(fs, name, dir, label, ID, ENTITY, results);

    // TS-specific leaf validation: each subdir must have .ts/.tsx files,
    // .gitkeep, or a modules/ dir (hex-in-hex)
    let dirs = topology_helpers::list_dir_names(fs, dir);
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
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!(
                    "{ENTITY} `{name}` subdirectory {label}/{subdir}/ has no .ts/.tsx files"
                ),
                format!(
                    "{ENTITY} `{name}` has `{label}/{subdir}/` but it contains no TypeScript files. \
                     Every subdirectory in a container folder must be a module with .ts/.tsx files, \
                     a hex-in-hex with its own `modules/` structure, or have a `.gitkeep` placeholder."
                ),
                Some(sub_path.display().to_string()),
                None,
                false,
            ));
        }
    }
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
// T-TOPOLOGY-02: TS import boundary violation
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
            results.push(CheckResult::from_parts(
                "T-TOPOLOGY-02".to_owned(),
                Severity::Error,
                "Hexagonal topology import boundary violation".to_owned(),
                format!(
                    "The `{}` layer imports from the `{}` layer: `{import_path}`. In hexagonal topology, \
                     imports must flow inward (adapters -> application -> ports -> domain). The `{}` layer \
                     must not depend on `{}`. Move shared types to a common layer, or invert the dependency \
                     using an interface/port.",
                    source_layer.label(),
                    target_layer.label(),
                    source_layer.label(),
                    target_layer.label(),
                ),
                Some(file_path.display().to_string()),
                Some(line_number),
                false,
            ));
        }
    }
}
