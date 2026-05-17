/// Rule: io layer types must not expose interface contracts.
mod io_contracts_in_types;
/// Aggregates per-rule checks for the apparch source checks family.
mod run;
/// Rule: types layer must not expose behavioral API.
mod types_public_surface;

#[cfg(feature = "checks")]
pub use run::check;
