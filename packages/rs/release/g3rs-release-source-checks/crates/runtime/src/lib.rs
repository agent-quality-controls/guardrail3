mod input_failures;
mod readme_quality;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
