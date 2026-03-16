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
