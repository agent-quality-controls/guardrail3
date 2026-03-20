//! RS-ARCH-01 helpers — re-exports shared arch utilities with RS-specific defaults.

use std::path::Path;

use crate::app::arch_helpers;
use crate::domain::report::CheckResult;
use crate::ports::outbound::FileSystem;

const ID: &str = "R-ARCH-01";
const ENTITY: &str = "Service";

pub fn list_dir_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    arch_helpers::list_dir_names(fs, dir)
}

pub fn list_file_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    arch_helpers::list_file_names(fs, dir)
}

pub fn has_gitkeep(fs: &dyn FileSystem, dir: &Path) -> bool {
    arch_helpers::has_gitkeep(fs, dir)
}

pub fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    arch_helpers::check_loose_files(fs, name, dir, label, ID, ENTITY, results);
}

pub fn check_exact_subdirs(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    expected: &[&str],
    results: &mut Vec<CheckResult>,
) {
    arch_helpers::check_exact_subdirs(fs, name, dir, label, expected, ID, ENTITY, results);
}

pub fn check_container_not_empty(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    arch_helpers::check_container_not_empty(fs, name, dir, label, ID, ENTITY, results);
}
