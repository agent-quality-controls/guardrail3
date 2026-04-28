mod content_adapter_astro_content;
mod content_adapter_exists;
mod content_adapter_rule;
mod eslint_disable_inventory;
mod inline_copy_rule;
mod pipeline_plugin_package_present;
mod policy_eslint_coverage;
mod protected_rule_disables_restricted;
mod route_scope_overlap;
mod run;
mod strict_content_policy;
mod strict_policy_paths;
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
