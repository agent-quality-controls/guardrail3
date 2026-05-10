//! Assertion helpers for the g3rs-arch config-checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    reason = "assertion modules follow a uniform layout (private ID/MESSAGE const + pub assert_* helpers that panic on mismatch); doc-per-private-item adds no signal and the panic surface is documented by the assertion name"
)]

#[cfg(feature = "checks")]
use g3rs_arch_config_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod dependency_count_split;
#[cfg(feature = "checks")]
pub mod feature_contract;
#[cfg(feature = "checks")]
pub mod no_boundary_crossing;
#[cfg(feature = "checks")]
pub mod shared_flag_required;
