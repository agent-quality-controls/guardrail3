#![allow(
    clippy::missing_docs_in_private_items,
    reason = "runtime is currently a scaffold; rule modules will add the real documented surface"
)]

mod rs_cargo_config_01_workspace_lints;
mod rs_cargo_config_02_lint_levels;
mod rs_cargo_config_03_workspace_metadata;
mod rs_cargo_config_04_priority_order;
mod rs_cargo_config_05_resolver;
mod rs_cargo_config_06_disallowed_macros_deny;
mod rs_cargo_config_07_approved_allow_inventory;
#[cfg(test)]
mod rs_cargo_config_07_approved_allow_inventory_tests;
mod rs_cargo_config_08_workspace_lints_inherited;
#[cfg(test)]
mod rs_cargo_config_08_workspace_lints_inherited_tests;
mod rs_cargo_config_09_no_weakened_overrides;
#[cfg(test)]
mod rs_cargo_config_09_no_weakened_overrides_tests;
mod rs_cargo_config_10_member_edition_drift;
#[cfg(test)]
mod rs_cargo_config_10_member_edition_drift_tests;
mod rs_cargo_config_11_unapproved_allow_entries;
#[cfg(test)]
mod rs_cargo_config_11_unapproved_allow_entries_tests;
mod rs_cargo_config_12_member_local_allows_forbidden;
#[cfg(test)]
mod rs_cargo_config_12_member_local_allows_forbidden_tests;
mod rs_cargo_config_13_rust_version_policy;
#[cfg(test)]
mod rs_cargo_config_13_rust_version_policy_tests;
mod run;
mod support;

#[cfg(test)]
use g3rs_cargo_config_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
