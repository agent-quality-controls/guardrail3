/// coverage exists module.
mod coverage_exists;
/// run module.
mod run;
/// same root conflict module.
mod same_root_conflict;

#[cfg(feature = "checks")]
pub use run::check;
