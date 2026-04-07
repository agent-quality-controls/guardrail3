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
