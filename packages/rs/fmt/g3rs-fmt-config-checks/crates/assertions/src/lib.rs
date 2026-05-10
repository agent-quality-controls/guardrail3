use g3rs_fmt_config_checks_runtime as _;

/// Shared helpers used by per-rule assertion modules in this crate.
mod common;

#[cfg(feature = "checks")]
pub mod edition_mismatch;
#[cfg(feature = "checks")]
pub mod extra_settings;
#[cfg(feature = "checks")]
pub mod ignore_escape_hatch;
#[cfg(feature = "checks")]
pub mod nightly_keys_on_stable;
#[cfg(feature = "checks")]
pub mod settings;
