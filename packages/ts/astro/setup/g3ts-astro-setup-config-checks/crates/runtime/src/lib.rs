mod run;
mod support;
mod ts_astro_config_01_astro_package_present;
mod ts_astro_config_02_astro_check_present;
mod ts_astro_config_03_astro_eslint_plugin_package_present;
mod ts_astro_config_05_astro_eslint_plugin_wired;
mod ts_astro_config_09_syncpack_stack_pins;
mod ts_astro_config_10_syncpack_forbidden_deps;
mod ts_astro_config_11_site_url;
mod ts_astro_config_12_static_output;
mod ts_astro_config_21_required_integrations;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
