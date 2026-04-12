#[cfg(test)]
use g3rs_deps_config_checks_assertions as _;

mod rs_deps_config_01_dependencies_allowlisted;
mod rs_deps_config_02_build_dependencies_allowlisted;
mod rs_deps_config_03_dev_dependencies_allowlisted;
mod rs_deps_config_04_library_allowlist_present;
mod rs_deps_config_05_direct_dependency_cap;
mod rs_deps_config_06_cargo_deny_installed;
mod rs_deps_config_07_cargo_machete_installed;
mod rs_deps_config_08_cargo_dupes_installed;
mod rs_deps_config_09_gitleaks_installed;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
