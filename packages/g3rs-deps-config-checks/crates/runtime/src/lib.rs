mod rs_deps_config_01_dependencies_allowlisted;
mod rs_deps_config_02_build_dependencies_allowlisted;
mod rs_deps_config_03_dev_dependencies_allowlisted;
mod rs_deps_config_04_library_allowlist_present;
mod rs_deps_config_05_direct_dependency_cap;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::{check_direct_dependency_cap, check_policy};
