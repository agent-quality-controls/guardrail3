mod baseline;
mod run;
mod support;
mod ts_eslint_config_01_exists;
mod ts_eslint_config_02_parseable;
mod ts_eslint_config_03_ts_plugin_present;
mod ts_eslint_config_04_project_service_enabled;
mod ts_eslint_config_05_no_explicit_any_error;
mod ts_eslint_config_06_no_console_error;
mod ts_eslint_config_07_thresholds;
mod ts_eslint_config_08_core_baseline_rules;
mod ts_eslint_config_09_type_safety_rules;
mod ts_eslint_config_10_hygiene_rules;
mod ts_eslint_config_11_unicorn_rules;
mod ts_eslint_config_12_regexp_rules;
mod ts_eslint_config_13_sonarjs_rules;
mod ts_eslint_config_14_test_relaxations;
mod ts_eslint_config_15_js_carveout;

#[cfg(feature = "checks")]
pub use run::check;
