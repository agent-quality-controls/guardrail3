#[cfg(test)]
use g3rs_release_repo_root_checks_assertions as _;
use g3rs_release_types as _;

mod release_plz_workflow;
mod publish_dry_run_workflow;
mod registry_token;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
