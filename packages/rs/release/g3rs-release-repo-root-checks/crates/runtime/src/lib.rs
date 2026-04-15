#[cfg(test)]
use g3rs_release_repo_root_checks_assertions as _;

mod rs_release_repo_root_01_release_plz_workflow;
mod rs_release_repo_root_02_publish_dry_run_workflow;
mod rs_release_repo_root_03_registry_token;
mod run;
mod support;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
