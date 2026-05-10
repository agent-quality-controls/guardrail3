//! Assertion helpers for the g3rs-apparch config-checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    reason = "assertion modules follow a uniform layout (private ID/MESSAGE const + pub assert_* helpers that panic on mismatch); doc-per-private-item adds no signal and the panic surface is documented by the assertion name"
)]

#[cfg(feature = "checks")]
use g3rs_apparch_config_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod dev_dependency_direction;
#[cfg(feature = "checks")]
pub mod io_outbound_dependency_direction;
#[cfg(feature = "checks")]
pub mod logic_dependency_direction;
#[cfg(feature = "checks")]
pub mod logic_purity;
#[cfg(feature = "checks")]
pub mod patch_replace_bypass;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod same_layer_cycles;
#[cfg(feature = "checks")]
pub mod types_dependency_direction;
#[cfg(feature = "checks")]
pub mod types_purity;
