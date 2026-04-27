mod run;
mod support;
mod ts_astro_config_17_pipeline_plugin_package_present;
mod ts_astro_config_18_content_adapter_rule;
mod ts_astro_config_19_inline_copy_rule;
mod ts_astro_config_23_strict_content_policy;
mod ts_astro_config_24_strict_policy_paths;
mod ts_astro_config_25_route_scope_overlap;
mod ts_astro_config_26_policy_eslint_coverage;
mod ts_astro_config_27_content_adapter_exists;
mod ts_astro_config_28_content_adapter_astro_content;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
