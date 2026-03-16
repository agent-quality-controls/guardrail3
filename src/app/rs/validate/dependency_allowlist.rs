use std::path::Path;

use crate::domain::config::types::CrateConfig;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// R-DEPS-01: Check that all dependencies are in the allowlist.
///
/// Parses the crate's Cargo.toml `[dependencies]` table and flags any dependency
/// not present in `allowed`. Skips `[dev-dependencies]`, `[build-dependencies]`,
/// and workspace path dependencies (deps with `path = "..."` pointing within the workspace).
pub fn check_dependency_allowlist(
    cargo_path: &Path,
    crate_name: &str,
    allowed: &[String],
    fs: &dyn FileSystem,
    results: &mut Vec<CheckResult>,
) {
    let Some(content) = fs.read_file(cargo_path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let Some(deps) = table.get("dependencies").and_then(|d| d.as_table()) else {
        return; // no [dependencies] section — nothing to check
    };

    for (dep_name, dep_value) in deps {
        // Skip workspace path dependencies (internal crates)
        if is_workspace_path_dep(dep_value) {
            continue;
        }

        if !allowed.iter().any(|a| a == dep_name) {
            results.push(CheckResult {
                id: "R-DEPS-01".to_owned(),
                severity: Severity::Error,
                title: format!("Unauthorized dependency in {crate_name}"),
                message: format!(
                    "Dependency `{dep_name}` is not in the allowed list for crate `{crate_name}`. \
                     Add it to `allowed_deps` in guardrail3.toml or remove the dependency."
                ),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
    }
}

/// Check if a dependency value indicates a workspace path dependency.
///
/// Matches both `dep = { path = "..." }` and `dep = { workspace = true }` forms.
fn is_workspace_path_dep(value: &toml::Value) -> bool {
    if let Some(tbl) = value.as_table() {
        // { path = "..." } — local path dependency
        if tbl.contains_key("path") {
            return true;
        }
        // { workspace = true } — workspace dependency (inherited from workspace Cargo.toml)
        if tbl.get("workspace").and_then(toml::Value::as_bool) == Some(true) {
            return true;
        }
    }
    false
}

/// R-DEPS-02: Warn when a library-profile crate has no dependency allowlist.
pub fn check_library_has_allowlist(
    crate_name: &str,
    crate_config: &CrateConfig,
    results: &mut Vec<CheckResult>,
) {
    let is_library = crate_config
        .profile
        .as_deref()
        .is_some_and(|p| p == "library");

    if is_library && crate_config.allowed_deps.is_none() {
        results.push(CheckResult {
            id: "R-DEPS-02".to_owned(),
            severity: Severity::Warn,
            title: format!("Library crate `{crate_name}` has no dependency allowlist"),
            message: format!(
                "Library crate `{crate_name}` has profile=\"library\" but no `allowed_deps`. \
                 Add `allowed_deps` to enforce least-privilege dependency control."
            ),
            file: None,
            line: None,
        });
    }
}

#[cfg(test)]
#[allow(clippy::indexing_slicing, clippy::expect_used)] // reason: test assertions
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Stub filesystem that returns a predefined Cargo.toml content.
    struct StubFs {
        content: String,
    }

    impl FileSystem for StubFs {
        fn read_file(&self, _path: &Path) -> Option<String> {
            Some(self.content.clone())
        }

        #[allow(clippy::unnecessary_wraps)] // reason: trait requires Result return type
        fn read_file_err(&self, _path: &Path) -> Result<String, std::io::Error> {
            Ok(self.content.clone())
        }

        fn list_dir(&self, _path: &Path) -> Vec<std::fs::DirEntry> {
            Vec::new()
        }

        fn metadata(&self, _path: &Path) -> Option<std::fs::Metadata> {
            None
        }
    }

    #[test]
    fn r_deps_01_unauthorized_dep_flagged() {
        let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"
tokio = "1"
thiserror = "1"
"#;
        let fs = StubFs {
            content: cargo_toml.to_owned(),
        };
        let mut results = Vec::new();
        let allowed = vec!["serde".to_owned(), "thiserror".to_owned()];

        check_dependency_allowlist(
            &PathBuf::from("crates/my-domain/Cargo.toml"),
            "my-domain",
            &allowed,
            &fs,
            &mut results,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "R-DEPS-01");
        assert_eq!(results[0].severity, Severity::Error);
        assert!(results[0].message.contains("tokio"));
    }

    #[test]
    fn r_deps_01_allowed_dep_passes() {
        let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"
thiserror = "1"
"#;
        let fs = StubFs {
            content: cargo_toml.to_owned(),
        };
        let mut results = Vec::new();
        let allowed = vec!["serde".to_owned(), "thiserror".to_owned()];

        check_dependency_allowlist(
            &PathBuf::from("crates/my-domain/Cargo.toml"),
            "my-domain",
            &allowed,
            &fs,
            &mut results,
        );

        assert!(results.is_empty());
    }

    #[test]
    fn r_deps_01_dev_deps_not_checked() {
        let cargo_toml = r#"
[package]
name = "my-domain"

[dependencies]
serde = "1"

[dev-dependencies]
tokio = { version = "1", features = ["test-util"] }
"#;
        let fs = StubFs {
            content: cargo_toml.to_owned(),
        };
        let mut results = Vec::new();
        let allowed = vec!["serde".to_owned()];

        check_dependency_allowlist(
            &PathBuf::from("crates/my-domain/Cargo.toml"),
            "my-domain",
            &allowed,
            &fs,
            &mut results,
        );

        assert!(results.is_empty());
    }

    #[test]
    fn r_deps_02_library_without_allowlist_warns() {
        let cfg = CrateConfig {
            layer: None,
            profile: Some("library".to_owned()),
            allowed_deps: None,
        };
        let mut results = Vec::new();

        check_library_has_allowlist("my-domain", &cfg, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "R-DEPS-02");
        assert_eq!(results[0].severity, Severity::Warn);
    }

    #[test]
    fn r_deps_02_service_without_allowlist_ok() {
        let cfg = CrateConfig {
            layer: None,
            profile: Some("service".to_owned()),
            allowed_deps: None,
        };
        let mut results = Vec::new();

        check_library_has_allowlist("my-api", &cfg, &mut results);

        assert!(results.is_empty());
    }

    #[test]
    fn r_deps_01_skips_workspace_path_deps() {
        let cargo_toml = r#"
[package]
name = "my-api"

[dependencies]
serde = "1"
my-domain = { path = "../my-domain" }
my-types = { workspace = true }
"#;
        let fs = StubFs {
            content: cargo_toml.to_owned(),
        };
        let mut results = Vec::new();
        let allowed = vec!["serde".to_owned()];

        check_dependency_allowlist(
            &PathBuf::from("crates/my-api/Cargo.toml"),
            "my-api",
            &allowed,
            &fs,
            &mut results,
        );

        // my-domain (path dep) and my-types (workspace dep) should be skipped
        assert!(results.is_empty());
    }
}
