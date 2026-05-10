//! Runtime rules for the `g3rs-garde-source-checks` family.

/// Rule implementation for `context validation surface`.
mod context_validation_surface;
/// Rule implementation for `enum derive validate`.
mod enum_derive_validate;
/// Rule implementation for `field level constraints`.
mod field_level_constraints;
/// Rule implementation for `input failures`.
mod input_failures;
/// Rule implementation for `manual deserialize impl`.
mod manual_deserialize_impl;
/// Rule implementation for `nested validation dive`.
mod nested_validation_dive;
/// Rule implementation for `query as inventory`.
mod query_as_inventory;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `struct derive validate`.
mod struct_derive_validate;
/// Internal support helpers shared by this crate's rules.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
