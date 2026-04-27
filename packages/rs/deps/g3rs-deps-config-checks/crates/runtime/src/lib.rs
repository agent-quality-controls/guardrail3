#[cfg(test)]
use g3rs_deps_config_checks_assertions as _;

mod build_dependencies_allowlisted;
mod cargo_deny_installed;
mod cargo_dupes_installed;
mod cargo_machete_installed;
mod dependencies_allowlisted;
mod dev_dependencies_allowlisted;
mod direct_dependency_cap;
mod gitleaks_installed;
mod library_allowlist_present;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
