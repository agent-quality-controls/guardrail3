mod eslint_suppression;
mod mdx_component_map_rule;
mod mdx_lane;
mod policy_helper_surfaces;
mod run;
mod strict_component_rules;
mod strict_policy_paths;
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
