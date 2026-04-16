/// Checks that root-level workspace lints are declared.
mod rs_cargo_config_01_workspace_lints;
/// Checks that lint levels use the approved strength ordering.
mod rs_cargo_config_02_lint_levels;
/// Checks that required workspace metadata keys are present.
mod rs_cargo_config_03_workspace_metadata;
/// Checks that workspace lint priority ordering is correct.
mod rs_cargo_config_04_priority_order;
/// Checks that the workspace resolver matches policy.
mod rs_cargo_config_05_resolver;
/// Checks that banned macro-related allows are denied.
mod rs_cargo_config_06_disallowed_macros_deny;
/// Inventories approved local allow entries.
mod rs_cargo_config_07_approved_allow_inventory;
/// Checks that members inherit workspace lints instead of restating them.
mod rs_cargo_config_08_workspace_lints_inherited;
/// Checks that member overrides do not weaken root policy.
mod rs_cargo_config_09_no_weakened_overrides;
/// Checks that member editions do not drift from workspace policy.
mod rs_cargo_config_10_member_edition_drift;
/// Checks that new allow entries are approved before use.
mod rs_cargo_config_11_unapproved_allow_entries;
/// Checks that members do not carry their own local allow lists.
mod rs_cargo_config_12_member_local_allows_forbidden;
/// Checks that member rust-version values match policy.
mod rs_cargo_config_13_rust_version_policy;
/// Orchestrates the cargo config rule fan-out.
mod run;
/// Shared matching tables and helper logic for the rule implementations.
mod support;

#[cfg(test)]
use g3rs_cargo_config_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
