use g3rs_release_repo_root_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_release_repo_root_01_release_plz_workflow;
#[cfg(feature = "checks")]
pub mod rs_release_repo_root_02_publish_dry_run_workflow;
#[cfg(feature = "checks")]
pub mod rs_release_repo_root_03_registry_token;
