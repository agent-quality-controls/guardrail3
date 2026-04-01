//! RS-TOPOLOGY-01 helpers — re-exports shared topology utilities with RS-specific defaults.

use std::path::Path;

use guardrail3_app_hexarch_helpers as hexarch_helpers;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::FileSystem;

const ID: &str = "R-TOPOLOGY-01";
const ENTITY: &str = "Service";

pub fn list_dir_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    hexarch_helpers::list_dir_names(fs, dir)
}

pub fn list_file_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    hexarch_helpers::list_file_names(fs, dir)
}

pub fn has_gitkeep(fs: &dyn FileSystem, dir: &Path) -> bool {
    hexarch_helpers::has_gitkeep(fs, dir)
}

pub fn is_gitkeep_only(fs: &dyn FileSystem, dir: &Path) -> bool {
    hexarch_helpers::is_gitkeep_only(fs, dir)
}

pub fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    hexarch_helpers::check_loose_files(fs, name, dir, label, ID, ENTITY, results);
}

pub fn check_exact_subdirs(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    expected: &[&str],
    results: &mut Vec<CheckResult>,
) {
    hexarch_helpers::check_exact_subdirs(fs, name, dir, label, expected, ID, ENTITY, results);
}

pub fn check_container_not_empty(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    hexarch_helpers::check_container_not_empty(fs, name, dir, label, ID, ENTITY, results);
}
