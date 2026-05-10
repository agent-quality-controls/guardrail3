//! Runtime rules for the `g3rs-fmt-config-checks` family.

/// Rule implementation for `edition mismatch`.
mod edition_mismatch;
/// Rule implementation for `extra settings`.
mod extra_settings;
/// Rule implementation for `ignore escape hatch`.
mod ignore_escape_hatch;
/// Input types for this family's rule checks.
mod inputs;
/// Rule implementation for `nightly keys on stable`.
mod nightly_keys_on_stable;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `settings`.
mod settings;

#[cfg(feature = "checks")]
pub use run::check;
