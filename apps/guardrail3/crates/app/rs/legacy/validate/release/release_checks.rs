use std::collections::BTreeMap;
use std::path::Path;

use guardrail3_app_core::discover::ProjectInfo;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

/// Information about a single discovered crate.
#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub(crate) name: String,
    pub(crate) cargo_toml_path: std::path::PathBuf,
    pub(crate) dir: std::path::PathBuf,
    pub(crate) publishable: bool,
    pub(crate) is_binary: bool,
    pub(crate) table: toml::Value,
}

/// Discover all crates by walking for Cargo.toml files.
pub fn discover_crates(fs: &dyn FileSystem, workspace_root: &Path) -> Vec<CrateInfo> {
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
    results.push(CheckResult::from_parts(
    "R-PUB-12".to_owned(),
    Severity::Info,
    "Crate inventory".to_owned(),
    format!(
            "Workspace has {} publishable crate(s) and {} with publish=false. Publishable crates will be checked for release metadata (description, license, readme, keywords, categories, semver). Informational inventory, no action needed.",
            publishable.len(),
            publish_false_count
        ),
    None,
    None,
    false,
    }.as_inventory());

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

    results,
)
