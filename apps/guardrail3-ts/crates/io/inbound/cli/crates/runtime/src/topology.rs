//! Topology family dispatch for the CLI: walks adopted TS units and runs
//! the file-tree-checks pipeline against each one. Also defines the shared
//! marker-pair filename constants used across the CLI runtime.

use std::path::{Path, PathBuf};

use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_ts_app_types::{FamilyResults, FamilyRunError, FamilyRunner, SupportedFamily};

use crate::fs as g3ts_fs;

/// Filename of the TS adoption marker that pairs with `package.json`.
pub(crate) const GUARDRAIL3_TS_TOML: &str = "guardrail3-ts.toml";
/// Filename of the npm/pnpm package manifest at an adopted TS unit root.
pub(crate) const PACKAGE_JSON: &str = "package.json";

/// CLI-local adapter that dispatches families into the bounded runner groups.
#[derive(Debug, Default)]
pub struct CliFamilyRunner;

impl FamilyRunner for CliFamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        crawl: &G3WorkspaceCrawl,
    ) -> Result<FamilyResults, FamilyRunError> {
        match family {
            SupportedFamily::Hooks => guardrail3_ts_family_runner_hooks::run(family, crawl),
            SupportedFamily::Topology => Ok(run_topology_family(crawl)),
            SupportedFamily::AstroSetup
            | SupportedFamily::AstroContent
            | SupportedFamily::AstroMdx
            | SupportedFamily::AstroI18n
            | SupportedFamily::AstroMedia
            | SupportedFamily::AstroSeo
            | SupportedFamily::AstroState
            | SupportedFamily::Arch
            | SupportedFamily::Apparch => guardrail3_ts_family_runner_structure::run(family, crawl),
            SupportedFamily::Eslint
            | SupportedFamily::Tsconfig
            | SupportedFamily::Package
            | SupportedFamily::Npmrc
            | SupportedFamily::Jscpd
            | SupportedFamily::Style
            | SupportedFamily::Fmt
            | SupportedFamily::Spelling
            | SupportedFamily::Typecov => guardrail3_ts_family_runner_config::run(family, crawl),
        }
    }
}

/// Runs the topology family across every adopted TS unit reachable from
/// the crawl root. Per-unit ingestion failures surface as one
/// `g3ts-topology/ingestion` error check instead of aborting the family,
/// so this function never returns an error condition itself.
fn run_topology_family(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let repo_root = crawl.root_abs_path.as_path();
    let mut results = Vec::new();
    for unit_root in adopted_ts_units(repo_root) {
        match g3ts_topology_ingestion::ingest_for_file_tree_checks(unit_root.as_path()) {
            Ok(input) => results.extend(g3ts_topology_file_tree_checks::check(&input)),
            Err(error) => results.push(G3CheckResult::new(
                "g3ts-topology/ingestion".to_owned(),
                G3Severity::Error,
                "topology ingestion failed".to_owned(),
                format!("{error:?}"),
                Some(unit_root.to_string_lossy().into_owned()),
                None,
            )),
        }
    }
    results
}

/// Walks `repo_root` and returns every directory containing both adoption markers.
fn adopted_ts_units(repo_root: &Path) -> Vec<PathBuf> {
    let mut units = Vec::new();
    collect_adopted_ts_units(repo_root, &mut units);
    units.sort();
    units
}

/// Depth-first walk of `dir` that appends every adopted-marker-pair directory
/// to `out`. Skips well-known generated/vendor directories.
fn collect_adopted_ts_units(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = g3ts_fs::read_dir_paths(dir);
    if entries.is_empty() {
        return;
    }
    let mut subdirs = Vec::new();
    let mut has_package_json = false;
    let mut has_g3ts_toml = false;
    for path in entries {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path.is_file() {
            if name == PACKAGE_JSON {
                has_package_json = true;
            } else if name == GUARDRAIL3_TS_TOML {
                has_g3ts_toml = true;
            }
            continue;
        }
        if !path.is_dir() {
            continue;
        }
        if matches!(
            name,
            "node_modules" | "target" | ".git" | ".cargo-target" | "dist" | "build"
        ) {
            continue;
        }
        subdirs.push(path);
    }
    if has_package_json && has_g3ts_toml {
        out.push(dir.to_path_buf());
    }
    for sub in subdirs {
        collect_adopted_ts_units(sub.as_path(), out);
    }
}
