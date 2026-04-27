mod run;
mod ts_astro_config_13_nuasite_checks;
mod ts_astro_config_14_sitemap_integration;
mod ts_astro_config_15_robots_integration;
mod ts_astro_config_16_llms_txt;
mod ts_astro_config_17_seo_packages;
mod ts_astro_config_22_structured_data_check;
mod ts_astro_config_24_strict_policy_paths;
mod ts_astro_config_29_policy_helper_surfaces;
mod ts_astro_config_31_metadata_helper_rule;
mod ts_astro_config_32_json_ld_helper_rule;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
