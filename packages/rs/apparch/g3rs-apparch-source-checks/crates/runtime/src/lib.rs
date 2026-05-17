//! Runtime rules for the `g3rs-apparch` family of source-level checks.

/// Rule that flags io-port traits accidentally placed in `*-types` packages.
mod io_traits_in_types;
/// Family entry point that runs all `g3rs-apparch` source-level rules.
mod run;
/// Rule that asserts the public surface of `*-types` packages matches the manifest declarations.
mod types_public_surface;

#[cfg(feature = "checks")]
pub use run::check;
