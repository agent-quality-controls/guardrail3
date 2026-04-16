mod rs_deny_config_13_unknown_sources_policy;
mod rs_deny_config_14_allow_registry_baseline;
mod rs_deny_config_15_allow_git_inventory;
mod rs_deny_config_18_skip_hygiene;
mod rs_deny_config_19_ignore_hygiene;
mod rs_deny_config_21_unknown_keys;
mod rs_deny_config_22_ignore_accumulation;
mod run;

pub(crate) use run::check;
