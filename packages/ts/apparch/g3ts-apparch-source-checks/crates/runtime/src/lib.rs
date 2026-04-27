mod io_contracts_in_types;
mod run;
mod types_public_surface;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_apparch_source_checks_assertions as _;
