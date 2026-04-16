#[cfg(test)]
use g3rs_apparch_source_checks_assertions as _;

mod rs_apparch_source_04_io_traits_in_types;
mod rs_apparch_source_05_types_public_surface;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
