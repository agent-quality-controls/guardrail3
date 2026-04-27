use g3rs_release_repo_root_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod release_plz_workflow;
#[cfg(feature = "checks")]
pub mod publish_dry_run_workflow;
#[cfg(feature = "checks")]
pub mod registry_token;
