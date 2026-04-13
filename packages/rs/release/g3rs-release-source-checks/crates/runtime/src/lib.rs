mod rs_release_source_01_readme_quality;
mod rs_release_source_02_input_failures;
mod run;
#[cfg(test)]
mod test_support;
#[cfg(test)]
use g3rs_release_source_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
