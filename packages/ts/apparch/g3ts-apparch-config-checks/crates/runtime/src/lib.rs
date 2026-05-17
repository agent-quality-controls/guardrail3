/// `app` package never depends directly on outbound infrastructure.
mod app_no_direct_outbound;
/// IO inbound packages depend in the correct direction.
mod io_inbound_dependency_direction;
/// IO outbound packages depend in the correct direction.
mod io_outbound_dependency_direction;
/// Logic packages depend in the correct direction.
mod logic_dependency_direction;
/// Logic packages remain pure (no IO).
mod logic_purity;
/// Top-level runtime entry point.
mod run;
/// Shared helpers across apparch rules.
mod support;
/// Types packages depend in the correct direction.
mod types_dependency_direction;
/// Types packages remain pure (no IO).
mod types_purity;

#[cfg(feature = "checks")]
pub use run::check;
