/// `astro_check_present` rule.
mod astro_check_present;
/// `astro_eslint_plugin_package_present` rule.
mod astro_eslint_plugin_package_present;
/// `astro_eslint_plugin_wired` rule.
mod astro_eslint_plugin_wired;
/// `astro_package_present` rule.
mod astro_package_present;
/// `eslint_comments_plugin_package_present` rule.
mod eslint_comments_plugin_package_present;
/// `eslint_disable_descriptions_required` rule.
mod eslint_disable_descriptions_required;
/// `forbidden_script_targets` rule.
mod forbidden_script_targets;
/// `lint_script` rule.
mod lint_script;
/// `protected_setup_rule_disables_restricted` rule.
mod protected_setup_rule_disables_restricted;
/// `required_integrations` rule.
mod required_integrations;
/// `run` rule.
mod run;
/// `site_url` rule.
mod site_url;
/// `static_output` rule.
mod static_output;
/// `support` rule.
mod support;
/// `syncpack_forbidden_deps` rule.
mod syncpack_forbidden_deps;
/// `syncpack_lint_script` rule.
mod syncpack_lint_script;
/// `syncpack_stack_pins` rule.
mod syncpack_stack_pins;
/// `unused_eslint_disables_fail` rule.
mod unused_eslint_disables_fail;
/// `validate_script` rule.
mod validate_script;

#[cfg(feature = "checks")]
pub use run::check;
