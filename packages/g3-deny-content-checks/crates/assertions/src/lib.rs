use g3_deny_content_checks_runtime as _;

mod common;

// Advisories
#[cfg(feature = "checks")]
pub mod rs_deny_04_deprecated_advisories;
#[cfg(feature = "checks")]
pub mod rs_deny_05_advisories_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_06_stricter_advisories_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_07_graph_all_features;
#[cfg(feature = "checks")]
pub mod rs_deny_08_graph_no_default_features;

// Bans
#[cfg(feature = "checks")]
pub mod rs_deny_10_multiple_versions_floor;
#[cfg(feature = "checks")]
pub mod rs_deny_11_highlight_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_12_allow_wildcard_paths;
#[cfg(feature = "checks")]
pub mod rs_deny_13_wildcards_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_21_tokio_full_ban;
#[cfg(feature = "checks")]
pub mod rs_deny_22_extra_feature_bans_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_27_duplicate_entries;

// Licenses
#[cfg(feature = "checks")]
pub mod rs_deny_14_license_allow_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_15_confidence_threshold;
#[cfg(feature = "checks")]
pub mod rs_deny_16_copyleft_allowlist;

// Sources
#[cfg(feature = "checks")]
pub mod rs_deny_18_unknown_sources_policy;
#[cfg(feature = "checks")]
pub mod rs_deny_19_allow_registry_baseline;
#[cfg(feature = "checks")]
pub mod rs_deny_20_allow_git_inventory;
#[cfg(feature = "checks")]
pub mod rs_deny_23_skip_hygiene;
#[cfg(feature = "checks")]
pub mod rs_deny_24_ignore_hygiene;
#[cfg(feature = "checks")]
pub mod rs_deny_28_unknown_keys;
#[cfg(feature = "checks")]
pub mod rs_deny_29_ignore_accumulation;
