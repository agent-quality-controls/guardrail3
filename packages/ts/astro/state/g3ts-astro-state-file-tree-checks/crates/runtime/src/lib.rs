mod run;
mod ts_astro_filetree_11_no_legacy_parallel_state;
mod ts_astro_filetree_12_configured_forbidden_state;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
