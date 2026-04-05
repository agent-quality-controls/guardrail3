mod common;

#[cfg(feature = "checks")]
pub mod rs_deps_05_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_06_build_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_07_dev_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod rs_deps_08_library_allowlist_present;
#[cfg(feature = "checks")]
pub mod rs_deps_12_direct_dependency_cap;
