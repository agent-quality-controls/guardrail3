#[cfg(test)]
use g3rs_release_repo_root_checks_assertions as _;
use g3rs_release_types as _;

/// `publish_dry_run_workflow` module.
mod publish_dry_run_workflow;
/// `registry_token` module.
mod registry_token;
/// `release_plz_workflow` module.
mod release_plz_workflow;
/// `run` module.
mod run;
/// `support` module.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
