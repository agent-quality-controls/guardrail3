//! Assertion helpers for the `g3rs-garde-source-checks` family's rule tests.

use g3rs_garde_source_checks_runtime as _;

/// Rule implementation for `common`.
mod common;

#[cfg(feature = "checks")]
pub mod context_validation_surface;
#[cfg(feature = "checks")]
pub mod enum_derive_validate;
#[cfg(feature = "checks")]
pub mod field_level_constraints;
#[cfg(feature = "checks")]
pub mod input_failures;
#[cfg(feature = "checks")]
pub mod manual_deserialize_impl;
#[cfg(feature = "checks")]
pub mod nested_validation_dive;
#[cfg(feature = "checks")]
pub mod query_as_inventory;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod struct_derive_validate;
