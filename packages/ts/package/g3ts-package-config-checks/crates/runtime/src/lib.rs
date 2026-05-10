/// Internal `local_banned_dependencies` module.
mod local_banned_dependencies;
/// Internal `root_engines` module.
mod root_engines;
/// Internal `root_exists` module.
mod root_exists;
/// Internal `root_package_manager` module.
mod root_package_manager;
/// Internal `root_parseable` module.
mod root_parseable;
/// Internal `root_pnpm` module.
mod root_pnpm;
/// Internal `root_private` module.
mod root_private;
/// Internal `root_scripts` module.
mod root_scripts;
/// Internal `run` module.
mod run;
/// Internal `support` module.
mod support;
/// Internal `validate_script_fail_closed` module.
mod validate_script_fail_closed;
/// Internal `validate_script_present` module.
mod validate_script_present;

#[cfg(feature = "checks")]
pub use run::check;
