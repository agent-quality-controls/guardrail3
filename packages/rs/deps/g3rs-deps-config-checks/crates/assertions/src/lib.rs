mod common;

#[cfg(feature = "checks")]
pub mod rs_deps_config_01_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_config_02_build_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_config_03_dev_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_config_04_library_allowlist_present;
#[cfg(feature = "checks")]
pub mod rs_deps_config_05_direct_dependency_cap;
#[cfg(feature = "checks")]
pub mod rs_deps_config_06_cargo_deny_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_config_07_cargo_machete_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_config_08_cargo_dupes_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_config_09_gitleaks_installed;
