use g3rs_deny_config_checks_runtime as _;

mod common;

// Advisories
#[cfg(feature = "checks")]
pub mod rs_deny_config_01_deprecated_advisories;
#[cfg(feature = "checks")]
pub mod rs_deny_config_02_advisories_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_config_03_stricter_advisories_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_04_graph_all_features;
#[cfg(feature = "checks")]
pub mod rs_deny_config_05_graph_no_default_features;

// Bans
#[cfg(feature = "checks")]
pub mod rs_deny_config_06_multiple_versions_floor;
#[cfg(feature = "checks")]
pub mod rs_deny_config_07_highlight_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_08_allow_wildcard_paths;
#[cfg(feature = "checks")]
pub mod rs_deny_config_09_wildcards_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_16_tokio_full_ban;
#[cfg(feature = "checks")]
pub mod rs_deny_config_17_extra_feature_bans_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_20_duplicate_entries;
#[cfg(feature = "checks")]
pub mod rs_deny_config_23_ban_baseline_complete;
#[cfg(feature = "checks")]
pub mod rs_deny_config_25_allow_override_channel;
#[cfg(feature = "checks")]
pub mod rs_deny_config_26_extra_deny_bans_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_27_wrappers;

// Licenses
#[cfg(feature = "checks")]
pub mod rs_deny_config_10_license_allow_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_config_11_confidence_threshold;
#[cfg(feature = "checks")]
pub mod rs_deny_config_12_copyleft_allowlist;
#[cfg(feature = "checks")]
pub mod rs_deny_config_24_license_exceptions_inventory;

// Sources
#[cfg(feature = "checks")]
pub mod rs_deny_config_13_unknown_sources_policy;
#[cfg(feature = "checks")]
pub mod rs_deny_config_14_allow_registry_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_config_15_allow_git_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_config_18_skip_hygiene;
#[cfg(feature = "checks")]
pub mod rs_deny_config_19_ignore_hygiene;
#[cfg(feature = "checks")]
pub mod rs_deny_config_21_unknown_keys;
#[cfg(feature = "checks")]
pub mod rs_deny_config_22_ignore_accumulation;
