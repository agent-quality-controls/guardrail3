use guardrail3_app_rs_family_cargo as _;

#[cfg(feature = "checks")]
pub mod rs_cargo_03_allow_inventory;
#[cfg(feature = "checks")]
pub mod rs_cargo_04_lint_inheritance;
#[cfg(feature = "checks")]
pub mod rs_cargo_06_no_weakened_overrides;
#[cfg(feature = "checks")]
pub mod rs_cargo_09_member_edition_drift;
#[cfg(feature = "checks")]
pub mod rs_cargo_10_missing_member_cargo;
#[cfg(feature = "checks")]
pub mod rs_cargo_12_unapproved_allow_entries;
#[cfg(feature = "checks")]
pub mod rs_cargo_13_member_local_allows_forbidden;
#[cfg(feature = "checks")]
pub mod rs_cargo_14_input_failures;
#[cfg(feature = "checks")]
pub mod rs_cargo_15_rust_version_policy;
