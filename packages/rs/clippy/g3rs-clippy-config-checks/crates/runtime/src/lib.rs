/// avoid breaking exported api module.
mod avoid_breaking_exported_api;
/// ban reason quality module.
mod ban_reason_quality;
/// baseline module.
mod baseline;
/// cognitive complexity threshold module.
mod cognitive_complexity_threshold;
/// config parseable module.
mod config_parseable;
/// duplicate bans module.
mod duplicate_bans;
/// excessive nesting threshold module.
mod excessive_nesting_threshold;
/// extra method ban module.
mod extra_method_ban;
/// extra type ban module.
mod extra_type_ban;
/// forbid clippy conf dir override module.
mod forbid_clippy_conf_dir_override;
/// library global state module.
mod library_global_state;
/// macro bans module.
mod macro_bans;
/// max fn params bools module.
mod max_fn_params_bools;
/// max struct bools module.
mod max_struct_bools;
/// missing method ban module.
mod missing_method_ban;
/// missing type ban module.
mod missing_type_ban;
/// policy context parseable module.
mod policy_context_parseable;
/// run module.
mod run;
/// support module.
mod support;
/// test relaxations module.
mod test_relaxations;
/// too many arguments threshold module.
mod too_many_arguments_threshold;
/// too many lines threshold module.
mod too_many_lines_threshold;
/// type complexity threshold module.
mod type_complexity_threshold;
/// unknown keys module.
mod unknown_keys;

#[cfg(feature = "checks")]
pub use run::check;
