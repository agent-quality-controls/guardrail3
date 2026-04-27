mod run;
mod support;
mod strict_component_rules;
mod ts_astro_config_20_mdx_lane;
mod ts_astro_config_24_strict_policy_paths;
mod ts_astro_config_29_policy_helper_surfaces;
mod ts_astro_config_30_mdx_component_map_rule;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
