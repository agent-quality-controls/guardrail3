/// `configured-forbidden-state` rule.
mod configured_forbidden_state;
/// `no-legacy-parallel-state` rule.
mod no_legacy_parallel_state;
/// Top-level runtime entry point.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
