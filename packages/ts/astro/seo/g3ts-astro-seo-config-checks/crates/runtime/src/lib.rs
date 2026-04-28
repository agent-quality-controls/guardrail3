mod artifact_validate_scripts;
mod broad_crawler_generator;
mod canonical_site_config;
mod crawler_checker_packages;
mod eslint_disable_inventory;
mod json_ld_helper_rule;
mod llms_integration_present;
mod metadata_helper_rule;
mod nuasite_checks;
mod nuasite_options;
mod policy_helper_surfaces;
mod protected_rule_disables_restricted;
mod robots_integration;
mod run;
mod seo_packages;
mod sitemap_integration;
mod static_output_config;
mod strict_policy_paths;
mod structured_data_check;
mod support;
mod trailing_slash_policy;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
