use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_ts_dir;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

// -----------------------------------------------------------------------
// T-ARCH-01: TS service missing hex arch structure
// -----------------------------------------------------------------------

/// Scan `apps/` for TypeScript apps and check that each has
/// `src/modules/domain/` and `src/modules/adapters/` subdirectories.
pub fn check_hex_arch_structure(
    fs: &dyn FileSystem,
    root: &Path,
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let apps = discover_ts_apps(fs, root);
    for app_dir in &apps {
        check_single_app_structure(fs, app_dir, &mut results);
    }
    results
}

/// Discover TypeScript apps under `<root>/apps/`.
/// An app is a subdirectory that has `package.json` or `src/`.
fn discover_ts_apps(fs: &dyn FileSystem, root: &Path) -> Vec<std::path::PathBuf> {
    let apps_dir = root.join("apps");
    let mut found = Vec::new();

    // Use list_dir to enumerate entries under apps/
    for entry in fs.list_dir(&apps_dir) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        // Check if it's a TS app (has package.json or src/)
        let has_package_json = fs.read_file(&path.join("package.json")).is_some();
        let has_src = fs.read_file(&path.join("src")).is_some()
            || path.join("src").is_dir();
        if has_package_json || has_src {
            found.push(path);
        }
    }
    found
}

/// Check a single TS app for hex arch structure.
fn check_single_app_structure(
    fs: &dyn FileSystem,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    let app_name = app_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let modules_dir = app_dir.join("src").join("modules");

    // Check if modules/ directory exists by probing for any known subdirectory
    let has_domain = dir_exists_via_probe(fs, &modules_dir.join("domain"));
    let has_adapters = dir_exists_via_probe(fs, &modules_dir.join("adapters"));

    if !has_domain || !has_adapters {
        let mut missing = Vec::new();
        if !has_domain {
            missing.push("domain");
        }
        if !has_adapters {
            missing.push("adapters");
        }
        results.push(CheckResult {
            id: "T-ARCH-01".to_owned(),
            severity: Severity::Warn,
            title: format!("TS app `{app_name}` missing hex arch layers"),
            message: format!(
                "App `{app_name}` is missing src/modules/{} subdirectories. \
                 Expected: src/modules/domain/, src/modules/adapters/",
                missing.join(", src/modules/"),
            ),
            file: Some(app_dir.display().to_string()),
            line: None,
        });
    }
}

/// Check if a directory likely exists by probing for common marker files
/// or by checking if any files can be read from it via walkdir.
fn dir_exists_via_probe(fs: &dyn FileSystem, dir: &Path) -> bool {
    // Try common markers
    let markers = ["index.ts", "index.tsx", "mod.ts", "types.ts"];
    for marker in &markers {
        if fs.read_file(&dir.join(marker)).is_some() {
            return true;
        }
    }
    // Fall back to checking if the directory itself exists on the real filesystem
    dir.is_dir()
}

// -----------------------------------------------------------------------
// T-ARCH-02: TS import boundary violation
// -----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TsLayer {
    Domain,
    Ports,
    Application,
    Adapters,
}

impl TsLayer {
    /// Returns the layers that this layer is NOT allowed to import from.
    const fn forbidden(self) -> &'static [Self] {
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
fn layer_from_path(path: &Path) -> Option<TsLayer> {
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

    // Handle relative imports: resolve ../.. segments
    if import_path.starts_with('.') {
        let resolved = resolve_relative(file_dir, import_path);
        return layer_from_resolved_path(&resolved);
    }

    None
}

/// Resolve a relative import path against the file's directory.
fn resolve_relative(base: &Path, rel: &str) -> String {
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
fn extract_import_path(line: &str) -> Option<&str> {
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
pub fn check_import_boundaries(
    fs: &dyn FileSystem,
    root: &Path,
) -> Vec<CheckResult> {
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
        // Only files inside a modules/ directory
        if path_str.contains("/modules/") {
            files.push(path_str);
        }
    }
    files
}

#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .ts/.tsx
fn is_ts_source(path: &str) -> bool {
    path.ends_with(".ts") || path.ends_with(".tsx")
}

/// Check a single file's imports for boundary violations.
fn check_file_imports(
    file_path: &Path,
    content: &str,
    results: &mut Vec<CheckResult>,
) {
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
                title: "Import boundary violation".to_owned(),
                message: format!(
                    "{} layer imports from {} layer: `{import_path}`",
                    source_layer.label(),
                    target_layer.label(),
                ),
                file: Some(file_path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

#[cfg(test)]
#[allow(clippy::indexing_slicing)] // reason: test assertions index into results
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::*;

    struct StubFs {
        files: BTreeMap<PathBuf, String>,
    }

    impl StubFs {
        fn new() -> Self {
            Self {
                files: BTreeMap::new(),
            }
        }
        fn add(&mut self, p: &str, c: &str) -> &mut Self {
            let _ = self.files.insert(PathBuf::from(p), c.to_owned());
            self
        }
    }

    impl FileSystem for StubFs {
        fn read_file(&self, path: &Path) -> Option<String> {
            self.files.get(path).cloned()
        }
        #[allow(clippy::unnecessary_wraps)] // reason: trait requires Result
        fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "stub"))
        }
        fn list_dir(&self, _: &Path) -> Vec<std::fs::DirEntry> {
            Vec::new()
        }
        fn metadata(&self, _: &Path) -> Option<std::fs::Metadata> {
            None
        }
    }

    // -------------------------------------------------------------------
    // T-ARCH-01 tests
    // -------------------------------------------------------------------

    #[test]
    fn t_arch_01_app_missing_modules_dir() {
        // Test the inner function directly since StubFs can't do list_dir
        let fs = StubFs::new();
        let app_dir = Path::new("/project/apps/my-app");
        let mut results = Vec::new();
        check_single_app_structure(&fs, app_dir, &mut results);
        assert_eq!(results.len(), 1, "expected 1 warning, got {results:?}");
        assert_eq!(results[0].id, "T-ARCH-01");
        assert!(matches!(results[0].severity, Severity::Warn));
        assert!(
            results[0].title.contains("my-app"),
            "should mention app name"
        );
    }

    #[test]
    fn t_arch_01_app_with_full_structure() {
        let mut fs = StubFs::new();
        let _ = fs.add(
            "/project/apps/my-app/src/modules/domain/index.ts",
            "export type User = { id: string };",
        );
        let _ = fs.add(
            "/project/apps/my-app/src/modules/adapters/index.ts",
            "export class DbAdapter {}",
        );
        let app_dir = Path::new("/project/apps/my-app");
        let mut results = Vec::new();
        check_single_app_structure(&fs, app_dir, &mut results);
        assert!(
            results.is_empty(),
            "expected no warnings, got: {results:?}"
        );
    }

    // -------------------------------------------------------------------
    // T-ARCH-02 tests
    // -------------------------------------------------------------------

    #[test]
    fn t_arch_02_domain_imports_adapters_fails() {
        let file_path =
            Path::new("/project/apps/my-app/src/modules/domain/user.ts");
        let content = "import { DbAdapter } from '../adapters/outbound/db';\n";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert_eq!(results.len(), 1, "expected 1 error, got {results:?}");
        assert_eq!(results[0].id, "T-ARCH-02");
        assert!(matches!(results[0].severity, Severity::Error));
        assert!(results[0].message.contains("domain"));
        assert!(results[0].message.contains("adapters"));
    }

    #[test]
    fn t_arch_02_domain_imports_application_fails() {
        let file_path =
            Path::new("/project/apps/my-app/src/modules/domain/types.ts");
        let content =
            "import { CreateUser } from '../application/commands/create-user';\n";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert_eq!(results.len(), 1, "expected 1 error, got {results:?}");
        assert_eq!(results[0].id, "T-ARCH-02");
        assert!(matches!(results[0].severity, Severity::Error));
        assert!(results[0].message.contains("domain"));
        assert!(results[0].message.contains("application"));
    }

    #[test]
    fn t_arch_02_application_imports_domain_ok() {
        let file_path = Path::new(
            "/project/apps/my-app/src/modules/application/commands/create-user.ts",
        );
        let content = "import { User } from '../../domain/types';\n";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert!(
            results.is_empty(),
            "application -> domain should be allowed, got: {results:?}"
        );
    }

    #[test]
    fn t_arch_02_application_imports_adapters_fails() {
        let file_path = Path::new(
            "/project/apps/my-app/src/modules/application/commands/create-user.ts",
        );
        let content = "import { db } from '../../adapters/outbound/db';\n";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert_eq!(results.len(), 1, "expected 1 error, got {results:?}");
        assert_eq!(results[0].id, "T-ARCH-02");
        assert!(matches!(results[0].severity, Severity::Error));
        assert!(results[0].message.contains("application"));
        assert!(results[0].message.contains("adapters"));
    }

    #[test]
    fn t_arch_02_adapters_imports_everything_ok() {
        let file_path = Path::new(
            "/project/apps/my-app/src/modules/adapters/outbound/db.ts",
        );
        let content = "\
import { User } from '../../domain/types';
import { UserRepo } from '../../ports/outbound/user-repo';
import { CreateUser } from '../../application/commands/create-user';
";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert!(
            results.is_empty(),
            "adapters should import from anything, got: {results:?}"
        );
    }

    #[test]
    fn t_arch_02_alias_import_detected() {
        let file_path =
            Path::new("/project/apps/my-app/src/modules/domain/user.ts");
        let content =
            "import { DbAdapter } from '@/modules/adapters/outbound/db';\n";
        let mut results = Vec::new();
        check_file_imports(file_path, content, &mut results);
        assert_eq!(results.len(), 1, "expected 1 error, got {results:?}");
        assert_eq!(results[0].id, "T-ARCH-02");
        assert!(matches!(results[0].severity, Severity::Error));
        assert!(results[0].message.contains("domain"));
        assert!(results[0].message.contains("adapters"));
    }

    // -------------------------------------------------------------------
    // Helper function unit tests
    // -------------------------------------------------------------------

    #[test]
    fn layer_from_path_detects_layers() {
        assert_eq!(
            layer_from_path(Path::new("/p/src/modules/domain/types.ts")),
            Some(TsLayer::Domain)
        );
        assert_eq!(
            layer_from_path(Path::new("/p/src/modules/ports/inbound/api.ts")),
            Some(TsLayer::Ports)
        );
        assert_eq!(
            layer_from_path(Path::new(
                "/p/src/modules/application/commands/create.ts"
            )),
            Some(TsLayer::Application)
        );
        assert_eq!(
            layer_from_path(Path::new("/p/src/modules/adapters/outbound/db.ts")),
            Some(TsLayer::Adapters)
        );
        assert_eq!(
            layer_from_path(Path::new("/p/src/utils/helper.ts")),
            None
        );
    }

    #[test]
    fn extract_import_path_various_forms() {
        assert_eq!(
            extract_import_path("import { X } from '../domain/types';"),
            Some("../domain/types")
        );
        assert_eq!(
            extract_import_path("import { X } from \"../domain/types\";"),
            Some("../domain/types")
        );
        assert_eq!(
            extract_import_path("const x = require('../domain/types');"),
            Some("../domain/types")
        );
        assert_eq!(
            extract_import_path("const x = require(\"../domain/types\");"),
            Some("../domain/types")
        );
        assert_eq!(extract_import_path("const x = 5;"), None);
    }

    #[test]
    fn resolve_relative_handles_parent() {
        let base = Path::new("/p/src/modules/domain");
        let resolved = resolve_relative(base, "../adapters/outbound/db");
        assert!(
            resolved.contains("modules/adapters"),
            "should resolve to modules/adapters, got: {resolved}"
        );
    }

    #[test]
    fn domain_forbidden_layers() {
        let forbidden = TsLayer::Domain.forbidden();
        assert!(forbidden.contains(&TsLayer::Application));
        assert!(forbidden.contains(&TsLayer::Adapters));
        assert!(forbidden.contains(&TsLayer::Ports));
    }

    #[test]
    fn ports_forbidden_layers() {
        let forbidden = TsLayer::Ports.forbidden();
        assert!(forbidden.contains(&TsLayer::Application));
        assert!(forbidden.contains(&TsLayer::Adapters));
        assert!(!forbidden.contains(&TsLayer::Domain));
    }

    #[test]
    fn application_forbidden_layers() {
        let forbidden = TsLayer::Application.forbidden();
        assert!(forbidden.contains(&TsLayer::Adapters));
        assert!(!forbidden.contains(&TsLayer::Domain));
        assert!(!forbidden.contains(&TsLayer::Ports));
    }

    #[test]
    fn adapters_forbidden_empty() {
        assert!(TsLayer::Adapters.forbidden().is_empty());
    }
}
