mod astro_check_present;
mod astro_eslint_plugin_package_present;
mod astro_eslint_plugin_wired;
mod astro_package_present;
mod lint_script;
mod required_integrations;
mod run;
mod site_url;
mod static_output;
mod support;
mod syncpack_forbidden_deps;
mod syncpack_lint_script;
mod syncpack_stack_pins;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod lib_tests;
