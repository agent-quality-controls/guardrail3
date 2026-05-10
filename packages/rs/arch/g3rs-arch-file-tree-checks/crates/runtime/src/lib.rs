//! Runtime crate for the g3rs-arch file-tree checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "rule modules in this crate follow a uniform layout (private const ID/SELECTOR + pub(crate) check fn); doc-per-private-item would add no signal beyond the module's public docstring"
)]

#[cfg(test)]
use g3rs_arch_file_tree_checks_assertions as _;

/// Rule: every crate root has a facade re-export module.
mod crate_has_facade;
/// Rule: each module directory contains a `mod.rs` file when required.
mod mod_rs_required;
/// Runtime orchestrator that wires inputs to rule checks.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
