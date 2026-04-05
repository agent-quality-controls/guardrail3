mod rs_deps_05_dependencies_allowlisted;
mod rs_deps_06_build_dependencies_allowlisted;
mod rs_deps_07_dev_dependencies_allowlisted;
mod rs_deps_08_library_allowlist_present;
mod rs_deps_12_direct_dependency_cap;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
