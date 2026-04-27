mod avoid_breaking_exported_api;
mod ban_reason_quality;
mod baseline;
mod cognitive_complexity_threshold;
mod config_parseable;
mod duplicate_bans;
mod excessive_nesting_threshold;
mod extra_method_ban;
mod extra_type_ban;
mod forbid_clippy_conf_dir_override;
mod library_global_state;
mod macro_bans;
mod max_fn_params_bools;
mod max_struct_bools;
mod missing_method_ban;
mod missing_type_ban;
mod policy_context_parseable;
mod run;
mod support;
mod test_relaxations;
mod too_many_arguments_threshold;
mod too_many_lines_threshold;
mod type_complexity_threshold;
mod unknown_keys;

#[cfg(feature = "checks")]
pub use run::check;
