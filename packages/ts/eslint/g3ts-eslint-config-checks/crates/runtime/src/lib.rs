/// Internal module `baseline`.
mod baseline;
/// Internal module `core_baseline_rules`.
mod core_baseline_rules;
/// Internal module `exists`.
mod exists;
/// Internal module `hygiene_rules`.
mod hygiene_rules;
/// Internal module `js_carveout`.
mod js_carveout;
/// Internal module `no_console_error`.
mod no_console_error;
/// Internal module `no_explicit_any_error`.
mod no_explicit_any_error;
/// Internal module `parseable`.
mod parseable;
/// Internal module `plugin_stack`.
mod plugin_stack;
/// Internal module `project_service_enabled`.
mod project_service_enabled;
/// Internal module `regexp_rules`.
mod regexp_rules;
/// Internal module `run`.
mod run;
/// Internal module `sonarjs_rules`.
mod sonarjs_rules;
/// Internal module `support`.
mod support;
/// Internal module `test_relaxations`.
mod test_relaxations;
/// Internal module `thresholds`.
mod thresholds;
/// Internal module `ts_plugin_present`.
mod ts_plugin_present;
/// Internal module `tsx_source_parity`.
mod tsx_source_parity;
/// Internal module `type_safety_rules`.
mod type_safety_rules;
/// Internal module `unicorn_rules`.
mod unicorn_rules;

#[cfg(feature = "checks")]
pub use run::check;
