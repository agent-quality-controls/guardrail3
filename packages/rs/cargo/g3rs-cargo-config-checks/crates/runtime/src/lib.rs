/// Inventories approved local allow entries.
mod approved_allow_inventory;
/// Checks that banned macro-related allows are denied.
mod disallowed_macros_deny;
/// Checks that lint levels use the approved strength ordering.
mod lint_levels;
/// Checks that member editions do not drift from workspace policy.
mod member_edition_drift;
/// Checks that members do not carry their own local allow lists.
mod member_local_allows_forbidden;
/// Checks that member overrides do not weaken root policy.
mod no_weakened_overrides;
/// Checks that workspace lint priority ordering is correct.
mod priority_order;
/// Checks that the workspace resolver matches policy.
mod resolver;
/// Orchestrates the cargo config rule fan-out.
mod run;
/// Checks that resolver 2 keeps rust-version-aware dependency fallback.
mod rust_version_aware_resolver;
/// Checks that member rust-version values match policy.
mod rust_version_policy;
/// Shared matching tables and helper logic for the rule implementations.
mod support;
/// Checks that new allow entries are approved before use.
mod unapproved_allow_entries;
/// Checks that root-level workspace lints are declared.
mod workspace_lints;
/// Checks that members inherit workspace lints instead of restating them.
mod workspace_lints_inherited;
/// Checks that required workspace metadata keys are present.
mod workspace_metadata;

#[cfg(feature = "checks")]
pub use run::check;
