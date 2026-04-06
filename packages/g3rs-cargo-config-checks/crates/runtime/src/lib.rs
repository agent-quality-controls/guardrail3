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
mod run;
mod support;

#[cfg(test)]
use g3rs_cargo_config_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
