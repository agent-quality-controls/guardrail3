mod common;

#[cfg(feature = "checks")]
use g3rs_deps_config_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod build_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod cargo_deny_installed;
#[cfg(feature = "checks")]
pub mod cargo_dupes_installed;
#[cfg(feature = "checks")]
pub mod cargo_machete_installed;
#[cfg(feature = "checks")]
pub mod dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod dev_dependencies_allowlisted;
#[cfg(feature = "checks")]
pub mod direct_dependency_cap;
#[cfg(feature = "checks")]
pub mod gitleaks_installed;
#[cfg(feature = "checks")]
pub mod library_allowlist_present;
