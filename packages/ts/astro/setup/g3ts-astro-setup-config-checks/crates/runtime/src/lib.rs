mod astro_check_present;
mod astro_eslint_plugin_package_present;
mod astro_eslint_plugin_wired;
mod astro_package_present;
mod eslint_comments_plugin_package_present;
mod eslint_disable_descriptions_required;
mod forbidden_script_targets;
mod lint_script;
mod protected_setup_rule_disables_restricted;
mod required_integrations;
mod run;
mod site_url;
mod static_output;
mod support;
mod syncpack_forbidden_deps;
mod syncpack_lint_script;
mod syncpack_stack_pins;
mod unused_eslint_disables_fail;
mod validate_script;

#[cfg(feature = "checks")]
pub use run::check;
