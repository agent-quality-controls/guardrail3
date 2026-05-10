//! Runtime rules for the `g3rs-deps-config-checks` family.

#[cfg(test)]
use g3rs_deps_config_checks_assertions as _;

/// Rule implementation for `build dependencies allowlisted`.
mod build_dependencies_allowlisted;
/// Rule implementation for `cargo deny installed`.
mod cargo_deny_installed;
/// Rule implementation for `cargo dupes installed`.
mod cargo_dupes_installed;
/// Rule implementation for `cargo machete installed`.
mod cargo_machete_installed;
/// Rule implementation for `dependencies allowlisted`.
mod dependencies_allowlisted;
/// Rule implementation for `dev dependencies allowlisted`.
mod dev_dependencies_allowlisted;
/// Rule implementation for `direct dependency cap`.
mod direct_dependency_cap;
/// Rule implementation for `gitleaks installed`.
mod gitleaks_installed;
/// Rule implementation for `library allowlist present`.
mod library_allowlist_present;
/// Family entry point that runs all rules.
mod run;
/// Internal support helpers shared by this crate's rules.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
