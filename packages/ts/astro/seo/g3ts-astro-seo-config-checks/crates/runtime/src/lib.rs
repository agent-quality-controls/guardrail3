mod json_ld_helper_rule;
mod llms_txt;
mod metadata_helper_rule;
mod nuasite_checks;
mod nuasite_options;
mod policy_helper_surfaces;
mod robots_integration;
mod run;
mod seo_packages;
mod sitemap_integration;
mod strict_policy_paths;
mod structured_data_check;
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
