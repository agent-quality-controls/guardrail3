mod run;
mod support;
mod ts_astro_config_01_astro_package_present;
mod ts_astro_config_02_astro_check_present;
mod ts_astro_config_03_astro_eslint_plugin_package_present;
mod ts_astro_config_04_render_validator_package_present;
mod ts_astro_config_06_pipeline_plugin_package_present;
mod ts_astro_config_07_pipeline_plugin_wired;

#[cfg(feature = "checks")]
pub use run::check;
