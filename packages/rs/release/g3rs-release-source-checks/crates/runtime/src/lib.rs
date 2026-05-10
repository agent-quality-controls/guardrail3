/// Input failure surfacing rule.
mod input_failures;
/// README quality rule.
mod readme_quality;
/// Top-level runtime entry point.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
