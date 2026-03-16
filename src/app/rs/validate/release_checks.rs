use std::collections::BTreeMap;
use std::path::Path;

use crate::app::discover::ProjectInfo;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::{FileSystem, ToolChecker};

/// Information about a single discovered crate.
#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub cargo_toml_path: std::path::PathBuf,
    pub dir: std::path::PathBuf,
    pub publishable: bool,
    pub is_binary: bool,
    pub table: toml::Value,
}

/// Discover all crates by walking for Cargo.toml files.
fn discover_crates(fs: &dyn FileSystem, workspace_root: &Path) -> Vec<CrateInfo> {
    let mut crates = Vec::new();

    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|e| !super::source_scan::is_excluded_dir(e))
        .flatten()
    {
        if entry.file_name() != "Cargo.toml" {
            continue;
        }
        // Skip test fixture Cargo.toml files — adversarial test data
        let entry_path_str = entry.path().display().to_string();
        if entry_path_str.contains("tests/fixtures/") {
            continue;
        }
        let path = entry.path();
        let Some(content) = fs.read_file(path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Value>() else {
            continue;
        };
        let Some(pkg) = table.get("package") else {
            continue; // workspace root without [package]
        };
        let name = pkg
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_owned();

        let publish_false = pkg
            .get("publish")
            .and_then(toml::Value::as_bool)
            .is_some_and(|b| !b);

        let has_bin_section = table.get("bin").and_then(|v| v.as_array()).is_some();
        let dir = path.parent().unwrap_or(workspace_root).to_path_buf();
        let has_main_rs = dir.join("src").join("main.rs").exists();

        crates.push(CrateInfo {
            name,
            cargo_toml_path: path.to_path_buf(),
            dir,
            publishable: !publish_false,
            is_binary: has_bin_section || has_main_rs,
            table,
        });
    }

    crates
}

/// Main orchestrator for release checks.
pub fn check(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    workspace_root: &Path,
    _project: &ProjectInfo,
    thorough: bool,
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let crates = discover_crates(fs, workspace_root);

    let publishable: Vec<&CrateInfo> = crates.iter().filter(|c| c.publishable).collect();
    let publish_false_count = crates.iter().filter(|c| !c.publishable).count();

    // Build a map of publishable crate names for cross-reference
    let publishable_names: std::collections::BTreeSet<String> =
        publishable.iter().map(|c| c.name.clone()).collect();

    // Build a map of crate name -> version for version consistency checks
    let version_map: BTreeMap<String, String> = publishable
        .iter()
        .filter_map(|c| {
            let ver = c
                .table
                .get("package")
                .and_then(|p| p.get("version"))
                .and_then(|v| v.as_str())
                .map(std::borrow::ToOwned::to_owned);
            ver.map(|v| (c.name.clone(), v))
        })
        .collect();

    // Per-crate checks
    for krate in &publishable {
        super::release_crate_checks::check_per_crate(
            fs,
            tc,
            krate,
            &publishable_names,
            &version_map,
            thorough,
            &mut results,
        );
    }

    // Inventory (R-PUB-12)
    results.push(CheckResult {
        id: "R-PUB-12".to_owned(),
        severity: Severity::Info,
        title: "Crate inventory".to_owned(),
        message: format!(
            "{} publishable, {} publish=false",
            publishable.len(),
            publish_false_count
        ),
        file: None,
        line: None,
    });

    // Repo-level checks
    super::release_repo_checks::check_repo_level(
        fs,
        tc,
        workspace_root,
        &publishable_names,
        &mut results,
    );

    // Binary checks
    for krate in &crates {
        if krate.is_binary {
            super::release_bin_checks::check_binary(fs, workspace_root, krate, &mut results);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub12_emits_inventory() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub12");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join("src"));
        let _ = std::fs::write(
            tmp.join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\ndescription = \"d\"\nlicense = \"MIT\"\nrepository = \"https://x\"",
        );
        let _ = std::fs::write(tmp.join("src/lib.rs"), "");

        let project = crate::app::discover::ProjectInfo {
            has_rust: true,
            has_typescript: false,
            cargo_workspace_root: Some(tmp.clone()),
            workspace_members: vec!["x".to_owned()],
            workspace_member_dirs: vec![".".to_owned()],
            package_json_path: None,
        };
        let tc = crate::adapters::outbound::tool_runner::RealToolChecker;
        let results = check(&fs, &tc, &tmp, &project, false);
        assert!(
            results
                .iter()
                .any(|c| c.id == "R-PUB-12" && c.severity == Severity::Info),
            "Should emit inventory: {results:?}"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
