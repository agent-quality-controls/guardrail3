mod content_config_exists;
mod live_config_exists;
mod no_route_markdown_pages;
mod no_velite_config;
mod no_velite_output;
mod run;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
