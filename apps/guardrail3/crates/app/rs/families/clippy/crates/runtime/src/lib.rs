#[cfg(feature = "api")]
pub mod clippy_support;
mod facts;
mod inputs;
mod rs_clippy_01_coverage;
mod rs_clippy_04_missing_method_ban;
mod rs_clippy_05_missing_type_ban;
mod rs_clippy_06_extra_method_ban;
mod rs_clippy_07_extra_type_ban;
mod rs_clippy_08_reason_quality;
mod rs_clippy_12_allowed_placement;
mod rs_clippy_13_local_policy_root_baseline;
mod rs_clippy_14_library_global_state;
mod rs_clippy_15_trivial_reason;
mod rs_clippy_16_avoid_breaking_exported_api;
mod rs_clippy_18_duplicate_bans;
mod rs_clippy_19_unknown_keys;
mod rs_clippy_20_macro_bans;
mod rs_clippy_23_policy_context_parseable;
mod rs_clippy_24_forbid_clippy_conf_dir_override;
mod rs_clippy_25_config_parseable;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_clippy_assertions as _;
