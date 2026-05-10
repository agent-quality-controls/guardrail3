//! Assertion helpers for the g3rs-arch file-tree checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    reason = "assertion modules follow a uniform layout (private ID/MESSAGE const + pub assert_* helpers that panic on mismatch); doc-per-private-item adds no signal and the panic surface is documented by the assertion name"
)]

#[cfg(feature = "checks")]
use g3rs_arch_file_tree_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod crate_has_facade;
#[cfg(feature = "checks")]
pub mod mod_rs_required;
