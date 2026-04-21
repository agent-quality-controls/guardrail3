mod run;
mod ts_apparch_source_01_types_public_surface;
mod ts_apparch_source_02_io_contracts_in_types;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_apparch_source_checks_assertions as _;
