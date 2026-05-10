/// Internal module `eslint_suppression`.
mod eslint_suppression;
/// Internal module `mdx_component_map_rule`.
mod mdx_component_map_rule;
/// Internal module `mdx_lane`.
mod mdx_lane;
/// Internal module `policy_helper_surfaces`.
mod policy_helper_surfaces;
/// Internal module `run`.
mod run;
/// Internal module `strict_component_rules`.
mod strict_component_rules;
/// Internal module `strict_policy_paths`.
mod strict_policy_paths;
/// Internal module `support`.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
