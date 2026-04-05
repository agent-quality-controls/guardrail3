#![allow(
    clippy::missing_docs_in_private_items,
    reason = "runtime is currently a scaffold; rule modules will add the real documented surface"
)]

mod rs_cargo_01_workspace_lints;
mod rs_cargo_02_lint_levels;
mod rs_cargo_05_workspace_metadata;
mod rs_cargo_07_priority_order;
mod rs_cargo_08_resolver;
mod rs_cargo_11_disallowed_macros_deny;
mod run;
mod support;

#[cfg(test)]
use g3_cargo_content_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
