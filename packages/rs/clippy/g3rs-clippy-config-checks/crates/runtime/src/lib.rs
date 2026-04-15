mod baseline;
mod rs_clippy_config_01_max_struct_bools;
mod rs_clippy_config_02_max_fn_params_bools;
mod rs_clippy_config_03_too_many_lines_threshold;
mod rs_clippy_config_04_too_many_arguments_threshold;
mod rs_clippy_config_05_excessive_nesting_threshold;
mod rs_clippy_config_06_test_relaxations;
mod rs_clippy_config_07_cognitive_complexity_threshold;
mod rs_clippy_config_08_type_complexity_threshold;
mod rs_clippy_config_09_missing_method_ban;
#[cfg(test)]
mod rs_clippy_config_09_missing_method_ban_tests;
mod rs_clippy_config_10_missing_type_ban;
#[cfg(test)]
mod rs_clippy_config_10_missing_type_ban_tests;
mod rs_clippy_config_11_extra_method_ban;
#[cfg(test)]
mod rs_clippy_config_11_extra_method_ban_tests;
mod rs_clippy_config_12_extra_type_ban;
#[cfg(test)]
mod rs_clippy_config_12_extra_type_ban_tests;
mod rs_clippy_config_13_ban_reason_quality;
#[cfg(test)]
mod rs_clippy_config_13_ban_reason_quality_tests;
mod rs_clippy_config_14_library_global_state;
#[cfg(test)]
mod rs_clippy_config_14_library_global_state_tests;
mod rs_clippy_config_15_avoid_breaking_exported_api;
#[cfg(test)]
mod rs_clippy_config_15_avoid_breaking_exported_api_tests;
mod rs_clippy_config_16_duplicate_bans;
#[cfg(test)]
mod rs_clippy_config_16_duplicate_bans_tests;
mod rs_clippy_config_17_unknown_keys;
#[cfg(test)]
mod rs_clippy_config_17_unknown_keys_tests;
mod rs_clippy_config_18_macro_bans;
#[cfg(test)]
mod rs_clippy_config_18_macro_bans_tests;
mod rs_clippy_config_19_policy_context_parseable;
#[cfg(test)]
mod rs_clippy_config_19_policy_context_parseable_tests;
mod rs_clippy_config_20_forbid_clippy_conf_dir_override;
#[cfg(test)]
mod rs_clippy_config_20_forbid_clippy_conf_dir_override_tests;
mod rs_clippy_config_21_config_parseable;
#[cfg(test)]
mod rs_clippy_config_21_config_parseable_tests;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
