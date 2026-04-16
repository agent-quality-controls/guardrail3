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
#[cfg(test)]
/// Attack cases for the approved allow inventory rule.
mod rs_cargo_config_07_approved_allow_inventory_tests;
/// Checks that members inherit workspace lints instead of restating them.
mod rs_cargo_config_08_workspace_lints_inherited;
#[cfg(test)]
/// Attack cases for the inherited workspace lints rule.
mod rs_cargo_config_08_workspace_lints_inherited_tests;
/// Checks that member overrides do not weaken root policy.
mod rs_cargo_config_09_no_weakened_overrides;
#[cfg(test)]
/// Attack cases for the weakened override rule.
mod rs_cargo_config_09_no_weakened_overrides_tests;
/// Checks that member editions do not drift from workspace policy.
mod rs_cargo_config_10_member_edition_drift;
#[cfg(test)]
/// Attack cases for the member edition drift rule.
mod rs_cargo_config_10_member_edition_drift_tests;
/// Checks that new allow entries are approved before use.
mod rs_cargo_config_11_unapproved_allow_entries;
#[cfg(test)]
/// Attack cases for the unapproved allow entries rule.
mod rs_cargo_config_11_unapproved_allow_entries_tests;
/// Checks that members do not carry their own local allow lists.
mod rs_cargo_config_12_member_local_allows_forbidden;
#[cfg(test)]
/// Attack cases for the member-local allow rule.
mod rs_cargo_config_12_member_local_allows_forbidden_tests;
/// Checks that member rust-version values match policy.
mod rs_cargo_config_13_rust_version_policy;
#[cfg(test)]
/// Attack cases for the rust-version policy rule.
mod rs_cargo_config_13_rust_version_policy_tests;
/// Orchestrates the cargo config rule fan-out.
mod run;
/// Shared matching tables and helper logic for the rule implementations.
mod support;

#[cfg(test)]
use g3rs_cargo_config_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
