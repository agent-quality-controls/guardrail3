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
                inventory: false,
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
            inventory: false,
        });
    }
}

