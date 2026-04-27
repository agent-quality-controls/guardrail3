mod run;
mod ts_astro_filetree_01_astro_config_exists;
mod ts_astro_filetree_03_live_config_exists;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
