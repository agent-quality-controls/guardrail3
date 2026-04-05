use guardrail3_app_rs_family_deps as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_deps_01_cargo_deny_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_02_cargo_machete_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_03_cargo_dupes_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_04_gitleaks_installed;
#[cfg(feature = "checks")]
pub mod rs_deps_09_cargo_lock_present;
#[cfg(feature = "checks")]
pub mod rs_deps_10_gitignore_not_ignoring_cargo_lock;
#[cfg(feature = "checks")]
pub mod rs_deps_11_input_failures;
