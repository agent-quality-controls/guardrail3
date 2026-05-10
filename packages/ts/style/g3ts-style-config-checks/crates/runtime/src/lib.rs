/// Internal `eslint_suppression` module.
mod eslint_suppression;
/// Internal `package_scripts` module.
mod package_scripts;
/// Internal `run` module.
mod run;
/// Internal `syncpack_policy` module.
mod syncpack_policy;

#[cfg(feature = "api")]
pub use run::check;
